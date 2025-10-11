use anyhow::{Context, Result};
use clap::{Arg, Command, crate_version};
use tracing::debug;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;

use synology_filestation_tools::{
    synology_api::{AuthenticationManager, Credential},
    users_dirs::{get_config_dir, get_current_exe_name_as_str},
};

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

#[tracing::instrument]
fn cli() -> Result<Command> {
    let exe_name = get_current_exe_name_as_str()?;

    let cmd = Command::new(exe_name)
        .version(crate_version!())
        .about("Synology FileStation FUSE mounter")
        .arg(
            Arg::new("url")
                .short('s')
                .long("url")
                .value_name("URL")
                .help("URL for Synology FileStation")
                .required(true),
        )
        .arg(
            Arg::new("user")
                .short('u')
                .long("user")
                .value_name("user")
                .help("Username for Synology FileStation")
                .required(true),
        )
        .arg(
            Arg::new("password")
                .short('p')
                .long("password")
                .value_name("PASSWORD")
                .help("Password for Synology FileStation")
                .required(false),
        );

    Ok(cmd)
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging()?;
    let matches = cli()?.get_matches();

    let url = matches
        .get_one::<String>("url")
        .context("Should have a url")?;

    let user = matches
        .get_one::<String>("user")
        .context("Should have a user")?;

    let auth_manager = AuthenticationManager::new(url.as_str(), user.as_str())?;
    let is_authenticated = auth_manager.is_authenticated()?;

    if !is_authenticated {
        let password = matches.get_one::<String>("password");

        let password = if let Some(password) = password {
            debug!("Using the provided password");
            password.to_owned()
        } else {
            debug!("Prompting for password");
            rpassword::prompt_password("Password: ")?
        };

        let crendential = Credential::new(user.to_owned(), password);
        auth_manager.login(&crendential).await;
    }

    Ok(())
}
