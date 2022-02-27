mod routes;
pub mod user;
use actix_web::{App, web, HttpServer, middleware};
use firebase_admin_auth_rs::jwk_auth::JwkAuth;
use std::env::var;

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    dotenv::from_filename(".env.dev").unwrap();

    pretty_env_logger::init();

    let auth = web::Data::new(JwkAuth::new(var("FIREBASE_PROJECT_ID").expect("Please set firebase project")).await);

    HttpServer
        ::new(move || App
            ::new()
            .route("/", web::get().to(routes::index))
            .route("/yes", web::get().to(routes::default))
            .route("/porn", web::get().to(routes::privileged))
            .app_data(auth.clone())
            .wrap(middleware::Logger::default())
        )
        .bind("0.0.0.0:8000")?
        .run().await
}
