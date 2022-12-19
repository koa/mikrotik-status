use async_graphql::SimpleObject;

use crate::config::CONFIG;

#[derive(SimpleObject)]
pub struct SettingsData {
    client_id: &'static str,
    token_url: String,
    auth_url: String,
}

impl Default for SettingsData {
    fn default() -> Self {
        SettingsData {
            client_id: &CONFIG.auth.client_id,
            auth_url: CONFIG.auth.get_auth_url(),
            token_url: CONFIG.auth.get_token_url(),
        }
    }
}
