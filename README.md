# Rest Api Using Rust

This is the simple api to create, update, delete, and get books using Rust and Actix.

### Database Setup
[Setup diesel](https://diesel.rs/guides/getting-started)

[Setup postgres](https://www.postgresql.org/download/)

### API Design
- GET Request

  ```
  curl http://localhost:5000/books
  ```
  
- POST Request
  
  ```
  curl -X POST -d '{"title": "<Book Title>", "author": "<Author>"}' -H "Content-type: application/json" http://localhost:5000/books
  ```
  
- PUT Request
  
  ```
  curl curl -X PUT -d '{"title": "<New Book Title>", "author": "<New Author>"}' -H "Content-type: application/json" http://localhost:5000/book/<id>
  ```
  
- DELETE Request
  
  ```
  curl -X DELETE http://localhost:5000/book/<id>
  ```
