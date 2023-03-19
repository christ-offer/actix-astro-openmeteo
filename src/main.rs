use actix_files as fs;
use actix_web::{App, HttpServer};

mod api;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
        .service(api::openmeteo::openmeteo)
        .service(
            fs::Files::new("/", "./www/dist")
                .index_file("index.html")
                .show_files_listing(),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
