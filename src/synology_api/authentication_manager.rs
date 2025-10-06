use crate::synology_api::authentication_database::AuthenticationDatabase;

pub struct AuthenticationManager;

impl AuthenticationManager {
    pub async fn authenticate(&self) {
        let database = AuthenticationDatabase { };

        panic!("Not implemented yet");
    }

    pub async fn login(&self, username: &str, password: &str, totp: Option<&str>) {
        panic!("Not implemented yet");
    }

    pub async fn logout(&self) {
        panic!("Not implemented yet");
    }

    pub fn is_authenticated(&self) -> bool {
        panic!("Not implemented yet");
    }
}