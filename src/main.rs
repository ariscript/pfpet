pub mod lib;
pub mod routes;

use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use env_logger;

use routes::discord;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/discord")
                    .service(discord::discord_user)
                    .service(discord::discord_emoji)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
