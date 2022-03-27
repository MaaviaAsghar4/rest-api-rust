extern crate actix_web;

use actix_web::{middleware, App, HttpServer};

use std::{env, io};

mod models;
mod routes;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");

    HttpServer::new(|| {
        App::new()
            .service(routes::get_all_books)
            .service(routes::get_book)
            .service(routes::insert_book)
            .service(routes::update_book)
            .service(routes::delete_book)
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
