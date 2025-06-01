mod services;

use actix_web::{App, HttpServer};

#[actix::main]
async fn main() -> std::io::Result<()> {
    const SERVER: &str = "127.0.0.1:8000";

    println!("Server running on http://{}", SERVER);

    HttpServer::new(move || App::new().configure(services::config))
        .bind(SERVER)?
        .run()
        .await
}
