use actix_web::{Responder, HttpRequest};
use super::user::RequestUser;

pub async fn index() -> impl Responder {
    "Hello world!"
}

pub async fn privileged(user: RequestUser) -> impl Responder {
    user.uid.to_string()
}

pub async fn default(req: HttpRequest) -> impl Responder {
    println!("{req:?}");

    "{\"message\": \"Go fuck your self\"}"
}