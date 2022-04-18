pub mod lib;
pub mod routes;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use std::env;

use routes::discord;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT environment variable must be a number.");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allowed_methods(["GET"]),
            )
            .service(web::scope("/d").service(discord::discord_user))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
