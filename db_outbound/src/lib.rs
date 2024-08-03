pub mod book_repository_impl;

pub use book_repository_impl::*;


pub fn init() {
    println!("DB Outbound library initialized");
}