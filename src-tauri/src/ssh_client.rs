use std::io::{Read, Write};
use std::net::TcpStream;

use ssh2::Session;

use crate::errors::{AppError, AppErrorCode};
use crate::models::ConnectPayload;

pub struct SshClient {
    session: Session,
}

impl SshClient {
    pub fn connect(payload: &ConnectPayload) -> Result<Self, AppError> {
        let host = payload.host.trim();
        let port = payload.port;

        if host.is_empty() {
            return Err(AppError::new(
                AppErrorCode::SshConnectFailed,
                "主机地址不能为空",
            ));
        }

        let addr = format!("{}:{}", host, port);
        let tcp = TcpStream::connect(&addr).map_err(|e| {
            let msg = e.to_string();
            if msg.contains("nodename") || msg.contains("servname") || msg.contains("lookup") {
                AppError::new(
                    AppErrorCode::SshConnectFailed,
                    format!("无法解析主机地址: {}", host),
                )
                .with_suggestion("请检查主机地址是否正确，确保可以 ping 通该地址")
            } else if msg.contains("Connection refused") {
                AppError::new(AppErrorCode::SshConnectFailed, "连接被拒绝")
                    .with_suggestion("请检查端口是否正确，确保 SSH 服务已启动")
            } else if msg.contains("timed out") || msg.contains("timeout") {
                AppError::new(AppErrorCode::SshConnectFailed, "连接超时")
                    .with_suggestion("请检查网络连通性，确保防火墙允许该端口")
            } else {
                AppError::new(AppErrorCode::SshConnectFailed, msg)
                    .with_suggestion("请检查主机地址、端口和网络连通性")
            }
        })?;

        let mut session = Session::new().map_err(|e| {
            AppError::new(AppErrorCode::SshConnectFailed, e.to_string())
                .with_suggestion("无法初始化 SSH 会话")
        })?;
        session.set_tcp_stream(tcp);
        session.handshake().map_err(|e| {
            AppError::new(AppErrorCode::SshConnectFailed, e.to_string())
                .with_suggestion("SSH 握手失败")
        })?;

        session
            .userauth_password(&payload.username, &payload.password)
            .map_err(|e| {
                AppError::new(AppErrorCode::SshAuthFailed, e.to_string())
                    .with_suggestion("请检查用户名或密码")
            })?;

        if !session.authenticated() {
            return Err(
                AppError::new(AppErrorCode::SshAuthFailed, "authentication failed")
                    .with_suggestion("请检查用户名或密码"),
            );
        }

        Ok(Self { session })
    }

    pub fn exec(&self, command: &str) -> Result<String, AppError> {
        let mut channel = self
            .session
            .channel_session()
            .map_err(|e| AppError::new(AppErrorCode::SshConnectFailed, e.to_string()))?;

        channel
            .exec(command)
            .map_err(|e| AppError::new(AppErrorCode::SshConnectFailed, e.to_string()))?;

        let mut out = String::new();
        channel
            .read_to_string(&mut out)
            .map_err(|e| AppError::new(AppErrorCode::SshConnectFailed, e.to_string()))?;
        channel
            .wait_close()
            .map_err(|e| AppError::new(AppErrorCode::SshConnectFailed, e.to_string()))?;
        Ok(out)
    }

    pub fn read_file(&self, path: &str) -> Result<String, AppError> {
        let escaped = escape_single_quotes(path);
        // First check if file exists to give a better error
        let check = self.exec(&format!("[ -f '{}' ] && echo 'exists'", escaped))?;
        if !check.contains("exists") {
            return Err(AppError::new(AppErrorCode::ConfigPathNotFound, format!("配置文件不存在: {}", path))
                .with_suggestion("请检查 V2Ray 安装路径或手动指定配置文件位置"));
        }

        self.exec(&format!("cat '{}'", escaped))
            .map_err(|e| {
                AppError::new(AppErrorCode::ConfigPermissionDenied, e.message)
                    .with_suggestion("请确保当前用户有该文件的读取权限")
            })
    }

    pub fn write_file_atomic(&self, path: &str, content: &str) -> Result<(), AppError> {
        let tmp = format!("{}.tmp", path);
        self.write_file(&tmp, content)?;
        self.exec(&format!(
            "mv '{}' '{}'",
            escape_single_quotes(&tmp),
            escape_single_quotes(path)
        ))
        .map_err(|e| AppError::new(AppErrorCode::RemoteUploadFailed, e.message))?;
        Ok(())
    }

    fn write_file(&self, path: &str, content: &str) -> Result<(), AppError> {
        let mut channel = self
            .session
            .scp_send(
                std::path::Path::new(path),
                0o644,
                content.len() as u64,
                None,
            )
            .map_err(|e| AppError::new(AppErrorCode::RemoteUploadFailed, e.to_string()))?;
        channel
            .write_all(content.as_bytes())
            .map_err(|e| AppError::new(AppErrorCode::RemoteUploadFailed, e.to_string()))?;
        channel
            .send_eof()
            .map_err(|e| AppError::new(AppErrorCode::RemoteUploadFailed, e.to_string()))?;
        channel
            .wait_eof()
            .map_err(|e| AppError::new(AppErrorCode::RemoteUploadFailed, e.to_string()))?;
        channel
            .close()
            .map_err(|e| AppError::new(AppErrorCode::RemoteUploadFailed, e.to_string()))?;
        Ok(())
    }
}

fn escape_single_quotes(input: &str) -> String {
    input.replace('\'', "'\\''")
}
