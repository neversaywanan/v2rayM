use serde::Serialize;

#[derive(Debug, Clone, Copy)]
pub enum AppErrorCode {
    SshAuthFailed,
    SshConnectFailed,
    ConfigPathNotFound,
    ConfigPermissionDenied,
    SubscriptionFetchFailed,
    RemoteUploadFailed,
    RemoteRestartFailed,
    RemoteRollbackFailed,
    JsonParseFailed,
    RemoteCommandFailed,
    General,
}

impl AppErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            AppErrorCode::SshAuthFailed => "SSH_AUTH_FAILED",
            AppErrorCode::SshConnectFailed => "SSH_CONNECT_FAILED",
            AppErrorCode::ConfigPathNotFound => "CONFIG_PATH_NOT_FOUND",
            AppErrorCode::ConfigPermissionDenied => "CONFIG_PERMISSION_DENIED",
            AppErrorCode::SubscriptionFetchFailed => "SUBSCRIPTION_FETCH_FAILED",
            AppErrorCode::RemoteUploadFailed => "REMOTE_UPLOAD_FAILED",
            AppErrorCode::RemoteRestartFailed => "REMOTE_RESTART_FAILED",
            AppErrorCode::RemoteRollbackFailed => "REMOTE_ROLLBACK_FAILED",
            AppErrorCode::JsonParseFailed => "JSON_PARSE_FAILED",
            AppErrorCode::RemoteCommandFailed => "REMOTE_COMMAND_FAILED",
            AppErrorCode::General => "GENERAL_ERROR",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AppError {
    pub code: String,
    pub message: String,
    pub suggestion: Option<String>,
}

impl AppError {
    pub fn new(code: AppErrorCode, message: impl Into<String>) -> Self {
        Self {
            code: code.as_str().to_string(),
            message: message.into(),
            suggestion: None,
        }
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for AppError {}
