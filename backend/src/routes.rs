use crate::user;
use actix_web::{HttpRequest, Responder};

use super::user::UserRequest;

pub async fn index() -> impl Responder {
    "Hello world!"
}

pub async fn privileged(user: UserRequest) -> impl Responder {
    ""
}

pub async fn default(req: HttpRequest) -> impl Responder {
    println!("{req:?}");

    "{\"message\": \"Go fuck your self\"}"
}
