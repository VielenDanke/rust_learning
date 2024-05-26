#[macro_use]
extern crate rocket;

use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, RwLock};

use rocket::{Response, serde, State};
use rocket::config::Config;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

#[get("/users")]
fn find_all(repository: &State<Repository>) -> Json<Vec<User>> {
    Json(repository.find_all())
}

#[post("/users", format = "json", data = "<user>")]
fn save(user: Json<User>, repository: &State<Repository>) -> ApiResponse {
    repository.save(user.name.clone());
    ApiResponse::build(None, Status::Created)
}

#[delete("/users/<id>")]
fn delete(id: u64, repository: &State<Repository>) -> ApiResponse {
    match repository.delete_by_id(id) {
        None => ApiResponse::build(None, Status::NotFound),
        Some(_) => ApiResponse::build(None, Status::Ok),
    }
}

#[get("/users/<id>")]
fn find_by_id(id: u64, repository: &State<Repository>) -> ApiResponse {
    match repository.find_by_id(id) {
        Some(user) => ApiResponse { json: Some(serde::json::to_string(&user).unwrap()), status: Status::Ok },
        None => ApiResponse { json: None, status: Status::NotFound },
    }
}

#[catch(404)]
fn not_found() -> &'static str {
    "Resource not found"
}

#[catch(500)]
fn internal_server_error() -> &'static str {
    "Internal server error"
}

#[derive(Debug)]
struct ApiResponse {
    json: Option<String>,
    status: Status,
}

impl ApiResponse {
    fn build(json: Option<String>, status: Status) -> ApiResponse {
        ApiResponse { json, status }
    }
}

impl<'r> Responder<'r, 'r> for ApiResponse {
    fn respond_to(self, req: &rocket::request::Request) -> rocket::response::Result<'r> {
        match self.json {
            None => Response::build()
                .status(self.status)
                .header(ContentType::JSON)
                .ok(),
            Some(json) => Response::build_from(json.respond_to(&req).unwrap())
                .status(self.status)
                .header(ContentType::JSON)
                .ok()
        }

    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct User {
    id: Option<u64>,
    name: String,
}

impl User {
    fn build(id: u64, name: String) -> User {
        User { id: Some(id), name }
    }
}

pub struct Repository {
    users: Arc<RwLock<Vec<User>>>,
}

impl Repository {
    fn new() -> Repository {
        Repository { users: Arc::new(RwLock::new(Vec::new())) }
    }

    fn find_by_id(&self, id: u64) -> Option<User> {
        self.users
            .read()
            .unwrap()
            .iter()
            .find_map(|v| {
                if v.id.unwrap() == id {
                    Some(v.clone())
                } else {
                    None
                }
            })
    }

    fn save(&self, name: String) {
        let next_id = self.users.read().unwrap().len() + 1;
        self.users.write().unwrap().push(User::build(next_id as u64, name));
    }

    fn find_all(&self) -> Vec<User> {
        self.users.read().unwrap().clone()
    }

    fn delete_by_id(&self, id: u64) -> Option<User> {
        match self.find_by_id(id) {
            None => None,
            Some(user) => {
                self.users.write().unwrap().remove((user.id.unwrap() - 1) as usize);
                Some(user)
            }
        }
    }
}

#[rocket::main]
async fn main() {
    let mut config = Config::default();
    config.address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    config.port = 8080;

    rocket::custom(config)
        .mount("/api/v2", routes![find_all, save, find_by_id, delete])
        .manage(Repository::new())
        .register("/", catchers![not_found, internal_server_error])
        .launch()
        .await
        .expect("Failed to launch Rocket server");
}
