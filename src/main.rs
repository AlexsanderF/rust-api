mod model;
mod schema;
mod services;

use actix_web::{App, HttpServer, middleware::Logger, web};
use dotenv::dotenv;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

struct AppState {
    db: Pool<Postgres>,
}

#[actix::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }

    dotenv().ok();
    env_logger::init();

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
            .wrap(Logger::default())
    })
    .bind(std::env::var("SERVER").unwrap())?
    .run()
    .await
}
