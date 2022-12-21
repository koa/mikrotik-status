use std::net::IpAddr;

use clap::Parser;
use lazy_static::lazy_static;

#[derive(Debug, Parser)]
pub struct Settings {
    /// client-id for oauth2
    #[arg(long, env = "AUTH_CLIENT_ID")]
    auth_client_id: String,
    /// issuer-url for oauth2
    #[arg(long, env = "AUTH_ISSUER")]
    auth_issuer: String,
    /// auth token url for oauth2 (default to "AUTH_ISSUER/protocol/openid-connect/token")
    #[arg(long, env = "AUTH_TOKEN_URL")]
    auth_token_url: Option<String>,
    /// auth token url for oauth2 (default to "AUTH_ISSUER/protocol/openid-connect/auth")
    #[arg(long, env = "AUTH_URL")]
    auth_url: Option<String>,

    /// webserver port
    #[arg(long, default_value = "8080", env = "SERVER_PORT")]
    server_port: u16,
    /// mgmt port (default: SERVER_PORT+1000)
    #[arg(long, env = "SERVER_MGMT_PORT")]
    server_mgmt_port: Option<u16>,
    /// Bind IP Address
    #[arg(long, default_value = "::1", env = "SERVER_BIND_ADDR")]
    server_bind_address: IpAddr,

    /// URL of netbox server
    #[arg(long, env = "NETBOX_ENDPOINT")]
    netbox_endpoint: String,
    /// Authentication token of netbox server
    #[arg(long, env = "NETBOX_TOKEN")]
    netbox_token: String,
}

impl Settings {
    pub fn auth_client_id(&self) -> &str {
        &self.auth_client_id
    }
    pub fn auth_issuer(&self) -> &str {
        &self.auth_issuer
    }
    pub fn auth_token_url(&self) -> String {
        self.auth_token_url
            .clone()
            .unwrap_or_else(|| format!("{}/protocol/openid-connect/token", self.auth_issuer))
    }
    pub fn auth_url(&self) -> String {
        self.auth_url
            .clone()
            .unwrap_or_else(|| format!("{}/protocol/openid-connect/auth", self.auth_issuer))
    }
    pub fn server_port(&self) -> u16 {
        self.server_port
    }
    pub fn server_mgmt_port(&self) -> u16 {
        self.server_mgmt_port
            .unwrap_or_else(|| self.server_port() + 1000)
    }
    pub fn server_bind_address(&self) -> &IpAddr {
        &self.server_bind_address
    }
    pub fn netbox_endpoint(&self) -> &str {
        &self.netbox_endpoint
    }
    pub fn netbox_token(&self) -> &str {
        &self.netbox_token
    }
}

lazy_static! {
    pub static ref CONFIG: Settings = Settings::parse();
}
pub fn config() -> &'static Settings {
    return &CONFIG;
}
