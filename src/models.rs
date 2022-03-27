use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use super::schema::books;
use super::schema::books::dsl::books as all_books;

#[derive(Serialize, Queryable, Debug, Deserialize)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
}

#[derive(Queryable, Insertable, Debug, Deserialize, Serialize)]
#[table_name = "books"]
pub struct NewBook {
    pub title: String,
    pub author: String,
}

impl Book {
    // show all books
    pub fn get_all_books(conn: &PgConnection) -> Vec<Book> {
        all_books
            .order(books::id.desc())
            .load::<Book>(conn)
            .expect("Error loading books")
    }

    // get a single book
    pub fn get_book(id: i32, conn: &PgConnection) -> Vec<Book> {
        all_books
            .find(id)
            .load::<Book>(conn)
            .expect("Error loading books")
    }

    // post a book
    pub fn insert_book(book: NewBook, conn: &PgConnection) -> bool {
        diesel::insert_into(books::table)
            .values(&book)
            .execute(conn)
            .is_ok()
    }

    // update a book
    pub fn update_book(id: i32, book: NewBook, conn: &PgConnection) -> bool {
        use super::schema::books::dsl::{author as a, title as t};
        let NewBook { title, author } = book;

        diesel::update(all_books.find(id))
            .set((a.eq(author), t.eq(title)))
            .get_result::<Book>(conn)
            .is_ok()
    }

    // delete a book
    pub fn delete_book(id: i32, conn: &PgConnection) -> bool {
        if Book::get_book(id, conn).is_empty() {
            return false;
        };
        diesel::delete(all_books.find(id)).execute(conn).is_ok()
    }
}
