use actix_web::{web, App, HttpServer};
mod handlers;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    
    // std::env::set_var("RUST_LOG", "actix_web=debug");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .route("/create_storage", web::get().to(handlers::create_storage))
            .route("/upload_file", web::get().to(handlers::upload_file))

            
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await
}