pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
}

pub struct NewBook {
    pub title: String,
    pub author: String,
}

impl Book {
    // show all books
    pub fn get_all_books() -> Vec<Book> {
        return vec![];
    }

    // get a single book
    pub fn get_book(id: i32) -> bool {
        false
    }

    // post a book
    pub fn insert_book(book: NewBook) -> bool {
        false
    }

    // update a book
    pub fn update_book(book: NewBook) -> bool {
        false
    }

    // delete a book
    pub fn delete_book(id: i32) -> bool {
        false
    }
}
