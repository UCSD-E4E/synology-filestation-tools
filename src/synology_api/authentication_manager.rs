use thiserror::Error;

use crate::synology_api::authentication_database::{
    AuthenticationDatabase, AuthenticationDatabaseError,
};

#[derive(Error, Debug)]
pub enum AuthenticationManagerError {
    #[error(transparent)]
    AuthenticationDatabaseError(#[from] AuthenticationDatabaseError),
}

#[derive(Debug)]
pub struct AuthenticationManager;

impl AuthenticationManager {
    #[tracing::instrument]
    pub async fn authenticate(&self) -> Result<(), AuthenticationManagerError> {
        let database = AuthenticationDatabase::new()?;

        panic!("Not implemented yet");
    }

    #[tracing::instrument]
    pub fn is_authenticated(&self) -> bool {
        panic!("Not implemented yet");
    }

    #[tracing::instrument]
    pub async fn login(&self) {
        panic!("Not implemented yet");
    }

    #[tracing::instrument]
    pub async fn logout(&self) {
        panic!("Not implemented yet");
    }
}
