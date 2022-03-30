extern crate actix_web;

#[macro_use]
extern crate diesel;

extern crate dotenv;

use actix_web::{App, HttpServer};

use std::{env, io};

mod db;
mod models;
mod routes;
mod schema;

// pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
// pub type DBPooledConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");

    HttpServer::new(move || {
        App::new()
            // .app_data(connection)
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
