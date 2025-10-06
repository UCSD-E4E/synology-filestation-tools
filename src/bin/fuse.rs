use synology_filestation_tools::synology_api::AuthenticationManager;

#[tokio::main]
async fn main() {
    let auth_manager = AuthenticationManager;

    auth_manager.authenticate().await;
}
