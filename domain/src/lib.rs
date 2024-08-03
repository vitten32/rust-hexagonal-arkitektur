pub mod models;
pub mod repositories;

pub use models::*;
pub use repositories::*;


pub fn init() {
    println!("Domain library initialized");
}