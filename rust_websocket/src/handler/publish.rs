use warp::http::StatusCode;
use warp::Reply;
use warp::ws::Message;

use crate::Clients;
use crate::Result;
use crate::structures::event::Event;

pub async fn publish_handler(body: Event, clients: Clients) -> Result<impl Reply> {
    clients
        .read()
        .unwrap()
        .iter()
        .filter(|(_, client)| match body.user_id {
            Some(v) => client.user_id == v,
            None => true,
        })
        .filter(|(_, client)| client.topics.contains(&body.topic))
        .for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let _ = sender.send(Ok(Message::text(body.message.clone())));
            }
        });

    Ok(StatusCode::OK)
}

pub async fn publish_topic_handler(body: Event, clients: Clients) -> Result<impl Reply> {
    clients
        .read()
        .unwrap()
        .iter()
        .filter(|(_, c)| c.topics.contains(&body.topic))
        .for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let _ = sender.send(Ok(Message::text(body.message.clone())));
            }
        });

    Ok(StatusCode::OK)
}
