use tauri_app_lib::{
    commands::resolve_config_path,
    errors::{AppError, AppErrorCode},
};

#[test]
fn fallback_uses_default_when_path_empty() {
    assert_eq!(resolve_config_path(None), "/etc/v2ray/config.json");
    assert_eq!(resolve_config_path(Some("  ")), "/etc/v2ray/config.json");
}

#[test]
fn maps_error_code_to_string() {
    let err = AppError::new(AppErrorCode::SshAuthFailed, "bad auth");
    assert_eq!(err.code, "SSH_AUTH_FAILED");
    assert_eq!(err.message, "bad auth");
}
