pub mod lib;
pub mod routes;

use actix_cors::Cors;
use actix_web::{dev::Service as _, middleware, web, App, HttpServer}; // need to bring the `Service` trait in scope
use actix_web::http::header;
use dotenv::dotenv;
use env_logger;
use tracing::Level;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::FmtSubscriber;
use std::env;

use routes::*;

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
                    .wrap_fn(|req, srv| {
                        let fut = srv.call(req);

                        async {
                            let mut res = fut.await?;
                            res.headers_mut().insert(header::CACHE_CONTROL, header::HeaderValue::from_str(&format!("max-age={}", discord::MAX_AGE))?);

                            Ok(res)
                        }
                    })
                    .service(discord::discord_user)
            )

            .service(
                web::scope("/gh")
                    .wrap_fn(|req, srv| {
                        let fut = srv.call(req);

                        async {
                            let mut res = fut.await?;
                            res.headers_mut().insert(header::CACHE_CONTROL, header::HeaderValue::from_str(&format!("max-age={}", github::MAX_AGE))?);

                            Ok(res)
                        }
                    })
                    .service(github::github_user)
            )
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
