#[derive(serde::Deserialize, serde::Serialize)]
pub struct RegisterRequest {
    pub user_id: usize,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RegisterResponse {
    pub url: String,
}
