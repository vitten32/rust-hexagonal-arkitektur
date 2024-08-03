use super::models::Book;

pub trait BookRepository {
    fn add_book(&self, book: Book) -> Result<(), String>;
    fn get_book(&self, id: u32) -> Result<Book, String>;
}