use uuid::Uuid;
use warp::http::StatusCode;
use warp::Reply;
use warp::reply::json;

use crate::Result;
use crate::Clients;
use crate::structures::client::Client;
use crate::structures::register::{RegisterRequest, RegisterResponse};

pub async fn register_handler(body: RegisterRequest, clients: Clients) -> Result<impl Reply> {
    let user_id = body.user_id;
    let uuid = Uuid::new_v4().simple().to_string();

    register_client(uuid.clone(), user_id, clients).await;
    Ok(json(&RegisterResponse {
        url: format!("ws://127.0.0.1:8000/ws/{}", uuid),
    }))
}

pub async fn unregister_handler(id: String, clients: Clients) -> Result<impl Reply> {
    clients.write().unwrap().remove(&id);
    Ok(StatusCode::OK)
}

async fn register_client(id: String, user_id: usize, clients: Clients) {
    clients.write().unwrap().insert(
        id,
        Client {
            user_id,
            topics: vec![String::from("cats")],
            sender: None,
        },
    );
}
