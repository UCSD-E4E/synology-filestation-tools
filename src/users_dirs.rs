use std::{env, fs::create_dir_all, path::PathBuf};

use app_dirs2::{AppDataType, AppInfo, app_root};
use thiserror::Error;
use tracing::info;

#[derive(Error, Debug)]
pub enum UsersDirsError {
    #[error(transparent)]
    AppDirsError(#[from] app_dirs2::AppDirsError),
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
}

#[tracing::instrument]
fn string_to_static_str(s: String) -> &'static str {
    // This is a really ugly hack to convert a String to a &'static str.
    // We are going to intentionally leak the string to make it static.
    Box::leak(s.into_boxed_str())
}

#[tracing::instrument]
fn get_app_info() -> Result<AppInfo, UsersDirsError> {
    let app_name = format!("engineers_for_exploration-{}", get_current_exe_name()?);

    Ok(AppInfo {
        name: string_to_static_str(app_name),
        author: "Engineers for Exploration",
    })
}

pub fn get_current_exe_name() -> Result<String, UsersDirsError> {
    let current_exe = env::current_exe()?;

    Ok(current_exe
        .file_stem()
        .expect("We should have a file name")
        .to_str()
        .expect("OS string should be UTF8")
        .to_string())
}

pub fn get_current_exe_name_as_str() -> Result<&'static str, UsersDirsError> {
    let current_exe = env::current_exe()?;

    Ok(string_to_static_str(
        current_exe
            .file_stem()
            .expect("We should have a file name")
            .to_str()
            .expect("OS string should be UTF8")
            .to_string(),
    ))
}

#[tracing::instrument]
pub fn get_config_dir() -> Result<PathBuf, UsersDirsError> {
    let path = app_root(AppDataType::UserConfig, &get_app_info()?)?;

    if !path.exists() {
        info!("Config directory does not exist, creating it.");
        create_dir_all(&path)?;
    }

    Ok(path)
}
