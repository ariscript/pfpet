pub mod lib;

use crate::lib::avatars::{discord::Discord, github::Github};
use crate::lib::avatars::AvatarFetch;
use crate::lib::filters::{bonk::Bonk, pet::Pet};
use crate::lib::handler::handler;
use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer}; // need to bring the `Service` trait in scope
use dotenv::dotenv;
use env_logger;
use std::env;
use tracing::Level;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let sub = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(sub)
        .expect("Setting tracing default subscriber failed.");

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT environment variable must be a number.");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(TracingLogger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allowed_methods(["GET"]),
            )
      
            .service(
                web::scope("/d")
                    .wrap(middleware::DefaultHeaders::new().add((
                        "Cache-Control",
                        format!("max-age={}", Discord::cache_max_length()),
                    )))
                    .service(handler("/{id}.gif", Discord, Pet))
                    .service(handler("/bonk/{id}.gif", Discord, Bonk)),
            )

            .service(
                web::scope("/gh")
                    .wrap(middleware::DefaultHeaders::new().add((
                        "Cache-Control",
                        format!("max-age={}", Github::cache_max_length()),
                    )))
                    .service(handler("/{username}.gif", Github, Pet)),
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
