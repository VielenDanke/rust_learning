use std::future::Future;

use warp::http::StatusCode;
use warp::Reply;

use crate::Result;

pub fn health_handler() -> impl Future<Output=Result<impl Reply>> {
    futures::future::ready(Ok(StatusCode::OK))
}
