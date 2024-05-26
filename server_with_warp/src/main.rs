use std::convert::Infallible;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use warp::{Filter, reject, Rejection, Reply};
use warp::http::StatusCode;

use crate::Error::NotFoundError;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Employee {
    id: Option<u64>,
    name: String,
}

async fn get_by_id(id: u64, employees: Arc<RwLock<Vec<Employee>>>) -> Result<impl Reply, Rejection> {
    let guard = employees.read()
        .unwrap();
    let opt_em = guard
        .iter()
        .find(|e| e.id.unwrap() == id);

    match opt_em {
        None => Err(reject::custom(NotFoundError)),
        Some(emp) => Ok(warp::reply::json(emp)),
    }
}

async fn find_all(employees: Arc<RwLock<Vec<Employee>>>) -> Result<impl Reply, Rejection> {
    let guard = employees.read().unwrap();
    Ok(warp::reply::json(&guard.clone()))
}

async fn create(mut employee: Employee, employees: Arc<RwLock<Vec<Employee>>>) -> Result<impl Reply, Rejection> {
    let mut guard = employees.write().unwrap();
    let next_id = (guard.len() + 1) as u64;
    employee.id = Some(next_id);
    guard.push(employee);
    Ok(warp::reply::json(&format!("User with ID {} is created", next_id)))
}

#[derive(Debug)]
enum Error {
    NotFoundError,
    InternalServerError,
}

impl reject::Reject for Error {}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if let Some(e) = err.find::<Error>() {
        match e {
            NotFoundError => {
                code = StatusCode::NOT_FOUND;
                message = "Couldn't found a requested source";
            }
            _ => {
                eprintln!("unhandled application error: {:?}", err);
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internal Server Error";
            }
        }
    } else if let Some(_) = err.find::<reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        eprintln!("unhandled error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    let json = warp::reply::json(&ErrorResponse {
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}

fn with_db(employees: Arc<RwLock<Vec<Employee>>>) -> impl Filter<Extract=(Arc<RwLock<Vec<Employee>>>, ), Error=Infallible> + Clone {
    warp::any().map(move || employees.clone())
}

#[tokio::main]
async fn main() {
    let employees = Arc::new(RwLock::new(Vec::new()));

    let find_by_id = warp::path!("employees" / u64)
        .and(warp::get())
        .and(with_db(employees.clone()))
        .and_then(get_by_id);

    let find_all = warp::path!("employees")
        .and(warp::get())
        .and(with_db(employees.clone()))
        .and_then(find_all);

    let save_new = warp::path!("employees")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(employees.clone()))
        .and_then(create);

    let api = save_new.or(find_by_id).or(find_all);

    let routes = api
        .with(warp::cors().allow_any_origin())
        .recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
