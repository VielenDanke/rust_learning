#[derive(serde::Deserialize, serde::Serialize)]
pub struct Event {
    pub topic: String,
    pub user_id: Option<usize>,
    pub message: String,
}
