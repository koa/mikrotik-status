use std::net::IpAddr;
use std::path::Path;

use config::{Config, Environment, File, FileFormat};
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub auth: Auth,
    pub server: Server,
    pub netbox: Netbox,
}

#[derive(Debug, Deserialize)]
pub struct Netbox {
    pub endpoint: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct Auth {
    pub client_id: String,
    pub issuer: String,
    token_url: Option<String>,
    auth_url: Option<String>,
}

impl Auth {
    pub fn get_token_url(&self) -> String {
        self.token_url
            .clone()
            .unwrap_or_else(|| format!("{}/protocol/openid-connect/token", self.issuer))
    }
    pub fn get_auth_url(&self) -> String {
        self.auth_url
            .clone()
            .unwrap_or_else(|| format!("{}/protocol/openid-connect/auth", self.issuer))
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct Server {
    port: Option<u16>,
    mgmt_port: Option<u16>,
    bind_address: Option<IpAddr>,
}

impl Server {
    pub fn get_port(&self) -> u16 {
        self.port.unwrap_or(8080)
    }
    pub fn get_mgmt_port(&self) -> u16 {
        self.mgmt_port.unwrap_or_else(|| self.get_port() + 1000)
    }
    pub fn get_bind_address(&self) -> IpAddr {
        self.bind_address.unwrap_or_else(|| IpAddr::from([0u8; 16]))
    }
}

impl Default for Settings {
    fn default() -> Self {
        let filename = "config.yaml";
        let mut config_builder = Config::builder();
        if Path::new(filename).exists() {
            config_builder = config_builder.add_source(File::new(filename, FileFormat::Yaml));
        }
        let settings = config_builder
            .add_source(Environment::default().separator("_"))
            .build()
            .expect("Cannot load config");
        settings.try_deserialize().expect("Cannot parse config")
    }
}
lazy_static! {
    pub static ref CONFIG: Settings = Default::default();
}
