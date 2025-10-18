use educe::Educe;
use reqwest::{Error, Response, StatusCode};
use thiserror::Error;
use tracing::info;
use urlencoding::encode;

use crate::{
    synology_api::authentication_database::{AuthenticationDatabase, AuthenticationDatabaseError},
    users_dirs::{UsersDirsError, get_current_exe_name},
};

#[derive(Educe)]
#[educe(Debug)]
pub struct Credential {
    pub username: String,
    #[educe(Debug(ignore))] // Do not include password in logs.
    pub password: String,
    pub device_id: Option<String>,
}

impl Credential {
    pub fn new(username: String, password: String) -> Self {
        Self::new_with_device_id(username, password, None)
    }

    pub fn new_with_device_id(
        username: String,
        password: String,
        device_id: Option<String>,
    ) -> Self {
        Self {
            username,
            password,
            device_id,
        }
    }
}

#[derive(Error, Debug)]
pub enum AuthenticationManagerError {
    #[error(transparent)]
    AuthenticationDatabaseError(#[from] AuthenticationDatabaseError),
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
    #[error(transparent)]
    UsersDirsError(#[from] UsersDirsError),
}

#[derive(Debug)]
pub struct AuthenticationManager<'a> {
    database: AuthenticationDatabase,
    url: &'a str,
    user: &'a str,
}

impl AuthenticationManager<'_> {
    #[tracing::instrument]
    pub fn new<'a>(
        url: &'a str,
        user: &'a str,
    ) -> Result<AuthenticationManager<'a>, AuthenticationManagerError> {
        let database = AuthenticationDatabase::new()?;

        Ok(AuthenticationManager {
            database,
            url,
            user,
        })
    }

    #[tracing::instrument]
    pub fn is_authenticated(&self) -> Result<bool, AuthenticationManagerError> {
        Ok(self.database.is_user_logged_in(self.url, self.user)?)
    }

    #[tracing::instrument]
    pub async fn login(
        &self,
        credential: &Credential,
        enable_device_token: bool,
        totp: Option<String>,
    ) -> Result<(), AuthenticationManagerError> {
        let device_name = format!(
            "{}::{}",
            hostname::get()?.to_string_lossy(),
            get_current_exe_name()?
        );

        let mut login_url = format!(
            "{}/webapi/entry.cgi?api=SYNO.API.Auth&version={}&method=login&account={}&passwd={}&device_name={}&session=FileStation&fromat=sid",
            self.url,
            6,
            encode(&credential.username),
            encode(&credential.password), // Encode the password in case it has characters not allowed in URLs in it.
            encode(&device_name)
        );

        if enable_device_token {
            login_url = format!("{}&enable_device_token=yes", login_url)
        }

        if let Some(did) = credential.device_id.clone() {
            info!("Credential has device ID");

            login_url = format!("{}&device_id={}", login_url, did)
        }

        if let Some(totp) = totp {
            info!("TOTP has been provided.");

            login_url = format!("{}&otp_code={}", login_url, totp)
        }

        let response: Result<Response, Error> = reqwest::get(login_url).await;
        panic!("Not implemented yet");
    }

    #[tracing::instrument]
    pub async fn logout(&self) {
        panic!("Not implemented yet");
    }
}
