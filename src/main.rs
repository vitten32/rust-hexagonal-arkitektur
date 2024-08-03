use bookstore_server::{init as main_init, run_server};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    main_init();
    run_server().await
}
