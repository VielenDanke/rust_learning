use tokio::sync::mpsc;
use warp::ws::Message;

#[derive(Clone)]
pub struct Client {
    pub user_id: usize,
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<Result<Message, warp::Error>>>
}
