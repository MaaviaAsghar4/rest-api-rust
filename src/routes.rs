use actix_web::{delete, get, post, put, HttpResponse};

#[get("/books")]
pub async fn get_all_books() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/book/<id>")]
pub async fn get_book() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/books")]
pub async fn insert_book() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[put("/book/<id>")]
pub async fn update_book() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[delete("/book/<id>")]
pub async fn delete_book() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}
