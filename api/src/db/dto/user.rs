#[derive(Debug, Clone, serde::Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SigninPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SafeUser {
    pub user_tag: Option<String>,
    pub username: String,
    pub email: String,
}
