use educe::Educe;
use thiserror::Error;

use crate::synology_api::authentication_database::{
    AuthenticationDatabase, AuthenticationDatabaseError,
};

#[derive(Educe)]
#[educe(Debug)]
pub struct Credential {
    pub user: String,
    #[educe(Debug(ignore))] // Do not include password in logs.
    pub password: String,
    pub totp: Option<String>,
}

#[derive(Error, Debug)]
pub enum AuthenticationManagerError {
    #[error(transparent)]
    AuthenticationDatabaseError(#[from] AuthenticationDatabaseError),
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
    pub async fn login(&self, credential: Credential) {
        panic!("Not implemented yet");
    }

    #[tracing::instrument]
    pub async fn logout(&self) {
        panic!("Not implemented yet");
    }
}
