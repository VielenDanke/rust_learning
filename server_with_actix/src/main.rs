use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use actix_web::{App, delete, get, HttpResponse, HttpServer, post, Responder, Result, web};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[get("/users")]
async fn find_all(datasource: web::Data<Users>) -> Result<impl Responder> {
    Ok(web::Json(datasource.read().unwrap().values().map(|u| u.clone()).collect::<Vec<User>>()))
}

#[get("/users/{id}")]
async fn find_by_id(path: web::Path<(String, )>, datasource: web::Data<Users>) -> HttpResponse {
    datasource.read().unwrap()
        .get(path.into_inner().0.as_str())
        .map(|u| HttpResponse::Ok().content_type(ContentType::json()).json(u))
        .or(Some(HttpResponse::NotFound().finish()))
        .unwrap()
}

#[delete("/users/{id}")]
async fn delete_by_id(path: web::Path<(String, )>, datasource: web::Data<Users>) -> HttpResponse {
    datasource.write().unwrap().remove(path.into_inner().0.as_str());
    HttpResponse::Ok().finish()
}

#[post("/users")]
async fn save(user: web::Json<User>, datasource: web::Data<Users>) -> HttpResponse {
    let created_user = User::build(user.0.username, user.0.password);
    let id_to_insert = created_user.id.as_ref().unwrap().clone();
    datasource.write().unwrap().insert(id_to_insert, created_user);
    HttpResponse::Created().finish()
}

#[derive(Clone, Serialize, Deserialize)]
struct User {
    id: Option<String>,
    username: String,
    password: String,
}

impl User {
    fn build(username: String, password: String) -> User {
        User {
            id: Some(Uuid::new_v4().to_string()),
            username,
            password,
        }
    }
}

type Users = Arc<RwLock<HashMap<String, User>>>;

#[actix_web::main]
async fn main() {
    let datasource: Users = Arc::new(RwLock::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api/v2")
                    .app_data(datasource.clone())
                    .service(find_all)
                    .service(find_by_id)
                    .service(delete_by_id)
                    .service(save)
            )
    })
        .keep_alive(Duration::from_secs(75))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
        .unwrap_or_else(|err| {
            eprintln!("Server error: {}", err);
            std::process::exit(1);
        })
}
