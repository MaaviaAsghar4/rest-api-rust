extern crate actix_web;

#[macro_use]
extern crate diesel;

extern crate dotenv;

use diesel::prelude::*;
use diesel::PgConnection;
use dotenv::dotenv;
use r2d2::{Pool, PooledConnection};

use actix_web::{App, HttpServer};

use std::{env, io};

mod models;
mod routes;
mod schema;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set properly");

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
