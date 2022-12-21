use async_graphql::SimpleObject;

use crate::config::config;
use crate::error;

#[derive(SimpleObject)]
pub struct SettingsData {
    client_id: &'static str,
    token_url: String,
    auth_url: String,
}

impl SettingsData {
    pub fn create_from_config() -> error::Result<Self> {
        let config = config();
        Ok(SettingsData {
            client_id: config.auth_client_id(),
            auth_url: config.auth_url(),
            token_url: config.auth_token_url(),
        })
    }
}
