
rust-hexagonal-architecture
Rust Hexagonal Architecture: Bookstore
This project demonstrates a hexagonal architecture in Rust for a Bookstore API. The project consists of several modules, each handling a specific part of the application, and uses actix-web to implement REST API endpoints.

Benefits of Using Rust for REST APIs
Performance: Rust is known for its high performance and low latency, making it ideal for APIs with stringent response time requirements.
Safety: Rust's strict type and ownership system helps prevent many common errors such as null pointer dereferences and data races.
Concurrency: Rust makes it easy to write concurrent code without sacrificing safety.
Project Structure
bookstore_api: Generated API code from the OpenAPI specification.
bookstore_server: The main application that starts the HTTP server and handles API requests.
db_outbound: Implements the repository interface by storing books in an in-memory list.
domain: Contains the business logic, including data models and the repository interface.
Step-by-Step Guide
1. Install Dependencies
Ensure you have Node.js and npm installed.

bash
   
npm install -g @openapitools/openapi-generator-cli
2. Generate API Code
Use the OpenAPI Generator to generate the Rust code for the API.

bash
   
openapi-generator-cli generate -i openapi/bookstore.yaml -g rust -o ./bookstore_api
3. Project Setup
Cargo.toml

makefile
   
[package]
name = "rust-hexagonal-architecture"
version = "0.1.0"
edition = "2018"

[dependencies]
actix-web = "4"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
domain = { path = "domain" }
db_outbound = { path = "db_outbound" }
bookstore_api = { path = "bookstore_api" }
4. Building Libraries
Domain Library (domain)

domain/src/lib.rs

rust
   
pub mod models;
pub mod repositories;

pub use models::*;
pub use repositories::*;

pub fn init() {
    println!("Domain library initialized");
}
domain/src/models.rs

rust
   
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
}
domain/src/repositories.rs

rust
   
use super::models::Book;

pub trait BookRepository {
    fn add_book(&self, book: Book) -> Result<(), String>;
    fn get_book(&self, id: u32) -> Result<Book, String>;
}
DB Outbound Library (db_outbound)

db_outbound/src/lib.rs

rust
   
pub mod book_repository_impl;

pub use book_repository_impl::*;

pub fn init() {
    println!("DB Outbound library initialized");
}
db_outbound/src/book_repository_impl.rs

rust
   
use domain::{Book, BookRepository};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct DbBookRepository {
    books: Arc<Mutex<Vec<Book>>>,
}

impl DbBookRepository {
    pub fn new() -> Self {
        DbBookRepository {
            books: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl BookRepository for DbBookRepository {
    fn add_book(&self, book: Book) -> Result<(), String> {
        let mut books = self.books.lock().map_err(|_| "Failed to lock mutex".to_string())?;
        books.push(book);
        Ok(())
    }

    fn get_book(&self, id: u32) -> Result<Book, String> {
        let books = self.books.lock().map_err(|_| "Failed to lock mutex".to_string())?;
        books.iter().find(|&book| book.id == id).cloned().ok_or_else(|| "Book not found".to_string())
    }
}
Bookstore API Library (bookstore_api)

bookstore_api/src/lib.rs

rust
   
// Generated code from OpenAPI specification
pub fn init() {
    println!("Bookstore API library initialized");
}
Bookstore Server Library (bookstore_server)

bookstore_server/src/lib.rs

rust
   
use actix_web::{web, App, HttpServer, HttpResponse};
use domain::{Book, BookRepository, init as domain_init};
use db_outbound::{DbBookRepository, init as db_outbound_init};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use bookstore_api::init as bookstore_api_init;

pub struct AppState {
    pub repository: Mutex<DbBookRepository>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateBookRequest {
    pub id: u32,
    pub title: String,
    pub author: String,
}

pub async fn add_book(data: web::Data<AppState>, book: web::Json<CreateBookRequest>) -> HttpResponse {
    let new_book = Book {
        id: book.id,
        title: book.title.clone(),
        author: book.author.clone(),
    };

    let repo = data.repository.lock().unwrap();
    repo.add_book(new_book).expect("Failed to add book");

    HttpResponse::Ok().json("Book added")
}

pub async fn get_book(data: web::Data<AppState>, book_id: web::Path<u32>) -> HttpResponse {
    let repo = data.repository.lock().unwrap();
    match repo.get_book(book_id.into_inner()) {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(_) => HttpResponse::NotFound().json("Book not found"),
    }
}

pub async fn run_server() -> std::io::Result<()> {
    let repository = DbBookRepository::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                repository: Mutex::new(repository.clone()),
            }))
            .route("/books", web::post().to(add_book))
            .route("/books/{id}", web::get().to(get_book))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

pub fn init() {
    domain_init();
    db_outbound_init();
    bookstore_api_init();
    println!("Main library initialized");
}
bookstore_server/src/main.rs

rust
   
use bookstore_server::{init as main_init, run_server};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    main_init();
    run_server().await
}
Running the Project
Navigate to the main application folder and build the project:

   
cargo build
To run the application:

arduino
   
cargo run
Testing the API
Test the API using curl or an API testing tool like Postman.

Add a book:

json
   
curl -X POST -H "Content-Type: application/json" -d '{"id": 1, "title": "Rust Programming", "author": "Steve Klabnik"}' http://127.0.0.1:8080/books
Get a book:

arduino
   
curl http://127.0.0.1:8080/books/1
Conclusion
This project demonstrates how to build an application using hexagonal architecture in Rust. By dividing functionality into different modules, the code becomes more maintainable and testable. Rust's performance and safety features make it an excellent choice for building reliable and efficient REST APIs.
