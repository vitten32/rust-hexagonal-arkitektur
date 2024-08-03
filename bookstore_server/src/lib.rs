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
