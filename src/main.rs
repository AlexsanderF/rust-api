mod model;
mod schema;
mod services;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

struct AppState {
    db: Pool<Postgres>,
}

#[actix::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connected to database");
            pool
        }
        Err(err) => {
            println!("Failed to connect to database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!(
        "Server running on http://{}",
        std::env::var("SERVER").unwrap()
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(services::config)
    })
    .bind(std::env::var("SERVER").unwrap())?
    .run()
    .await
}
