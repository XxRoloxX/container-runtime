use super::process::get_install_path;

pub fn get_log4rs_config_path() -> Result<String, String> {
    let install_path = get_install_path()?;
    Ok(format!("{}/log4rs.yaml", install_path))
}

pub fn configure_logging() -> Result<(), String> {
    let log_config_path = get_log4rs_config_path()?;
    log4rs::init_file(log_config_path, Default::default()).map_err(|e| e.to_string())
}
