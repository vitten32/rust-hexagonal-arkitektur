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
