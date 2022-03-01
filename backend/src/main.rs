mod routes;
pub mod user;

use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, HttpServer};
use log::{error};
use std::{env::var};
use std::time::Duration;
use std::sync::Mutex;

use jsonwebtoken::jwk::JwkSet;

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    dotenv::from_filename(".env.dev").unwrap();

    pretty_env_logger::init();

    let authority = var("AUTHZ_AUTHORITY").expect("Please set the `AUTHZ_AUTHORITY` environment variable");

    let jwks: web::Data<Mutex<JwkSet>> = web::Data::new(match reqwest::get(&format!("{authority}.well-known/jwks.json")).await {
        Ok(response) => {
            let text = response.text().await.unwrap();
            Mutex::new(serde_json::from_str(&text).unwrap())
        }
        Err(err) => {
            error!("Error fetching jwks {err}");
            panic!("");
        }
    });

    let jwks2 = jwks.clone();

    tokio::spawn(async move {
        let authority = var("AUTHZ_AUTHORITY").expect("Please set the `AUTHZ_AUTHORITY` environment variable");
        loop {
            match reqwest::get(&format!("{authority}.well-known/jwks.json")).await {
                Ok(response) => {
                    let text = response.text().await.unwrap();
                    let mut lock = jwks2.lock().unwrap();
                    *lock = serde_json::from_str(&text).unwrap();
                }
                Err(err) => {
                    error!("Error fetching jwks {err}")
                }
            }


            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(jwks.clone())
            .route("/", web::get().to(routes::index))
            .route("/yes", web::get().to(routes::default))
            .route("/porn", web::get().to(routes::privileged))
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(middleware::Logger::default())
    })
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
