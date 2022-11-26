use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct UserInfo {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub name: String,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
}
