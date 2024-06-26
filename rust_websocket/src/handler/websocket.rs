use futures::StreamExt;
use futures::FutureExt;
use serde_json::from_str;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{
    Reply,
    ws::{
        WebSocket, Ws,
    },
};
use warp::ws::Message;

use crate::{Clients, Result};
use crate::structures::client::Client;
use crate::structures::topic::TopicsRequest;

pub async fn ws_handler(ws: Ws, id: String, clients: Clients) -> Result<impl Reply> {
    let client = clients.read().unwrap().get(&id).cloned();
    match client {
        Some(c) => Ok(ws.on_upgrade(move |socket| client_connection(socket, id, clients, c))),
        None => Err(warp::reject::not_found()),
    }
}

pub async fn client_connection(ws: WebSocket, id: String, clients: Clients, mut client: Client) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();

    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));

    client.sender = Some(client_sender);

    clients.write().unwrap().insert(id.clone(), client);

    println!("Client {} is connected", id);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("error receiving ws message for id: {}): {}", id.clone(), e);
                break;
            }
        };
        client_msg(&id, msg, clients.clone()).await;
    }

    clients.write().unwrap().remove(&id);

    println!("Client {} disconnected", id);
}

async fn client_msg(id: &str, msg: Message, clients: Clients) {
    println!("Received message from {}: {:?}", id, msg);

    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    if message == "ping" || message == "ping\n" {
        return;
    }

    let topics_req: TopicsRequest = match from_str(&message) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error while parsing message to topics request: {}", e);
            return;
        }
    };

    let mut locked = clients.write().unwrap();

    match locked.get_mut(id) {
        Some(v) => {
            v.topics = topics_req.topics;
        }
        None => return,
    };
}
