use super::models::{Book, NewBook};
use crate::DBPooledConnection;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::Value;

#[get("/books")]
pub async fn get_all_books(conn: &DBPooledConnection) -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/book/{id}")]
pub async fn get_book(conn: &DBPooledConnection) -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/books")]
pub async fn insert_book(conn: &DBPooledConnection) -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[put("/book/{id}")]
pub async fn update_book(conn: &DBPooledConnection) -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[delete("/book/{id}")]
pub async fn delete_book(conn: &DBPooledConnection) -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}
