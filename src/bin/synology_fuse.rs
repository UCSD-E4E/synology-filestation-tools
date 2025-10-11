use anyhow::Result;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;

use synology_filestation_tools::{synology_api::AuthenticationManager, users_dirs::get_config_dir};

fn setup_logging() -> Result<()> {
    let config_path = get_config_dir()?;
    let log_file = rolling::daily(config_path, "log").with_max_level(tracing::Level::INFO);

    tracing_subscriber::fmt()
        .pretty()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_writer(log_file)
        .init();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging()?;

    let auth_manager = AuthenticationManager::new("", "")?;
    let is_authenticated = auth_manager.is_authenticated()?;

    if !is_authenticated {
        println!("User is not authenticated");
    }

    Ok(())
}
