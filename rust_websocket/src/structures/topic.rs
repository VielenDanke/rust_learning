#[derive(serde::Deserialize, serde::Serialize)]
pub struct TopicsRequest {
    pub topics: Vec<String>,
}
