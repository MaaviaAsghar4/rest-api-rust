use super::db;
use super::models::{Book, NewBook};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};

#[get("/books")]
pub async fn get_all_books() -> HttpResponse {
    let connection = db::establish_connection();
    let books = Book::get_all_books(&connection);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(books)
}

#[get("/book/{id}")]
pub async fn get_book(req: HttpRequest) -> HttpResponse {
    let connection = db::establish_connection();
    let id: i32 = req.match_info().query("id").parse().unwrap();
    let books = Book::get_book(id, &connection);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(books)
}

#[post("/books")]
pub async fn insert_book(new_book: web::Json<NewBook>) -> HttpResponse {
    let connection = db::establish_connection();
    let new_book = new_book.into_inner();
    let add_book = NewBook {
        title: new_book.title,
        author: new_book.author,
    };
    let books = Book::insert_book(add_book, &connection);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(books)
}

#[put("/book/{id}")]
pub async fn update_book(req: HttpRequest, new_book: web::Json<NewBook>) -> HttpResponse {
    let connection = db::establish_connection();
    let new_book = new_book.into_inner();
    let id: i32 = req.match_info().query("id").parse().unwrap();
    let add_book = NewBook {
        title: new_book.title,
        author: new_book.author,
    };
    let books = Book::update_book(id, add_book, &connection);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(books)
}

#[delete("/book/{id}")]
pub async fn delete_book(req: HttpRequest) -> HttpResponse {
    let connection = db::establish_connection();
    let id: i32 = req.match_info().query("id").parse().unwrap();
    let books = Book::delete_book(id, &connection);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(books)
}
