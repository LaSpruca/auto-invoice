use firebase_admin_auth_rs::jwk_auth::JwkAuth;

use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::Payload, web::Data, Error, FromRequest, HttpRequest, Result};
use futures_util::future::{err, ok, Ready};
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct RequestUser {
    pub uid: String,
}

impl FromRequest for RequestUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = match req.headers().get("Authorization") {
            Some(auth_header) => match auth_header.to_str() {
                Ok(v) => get_token_from_header(v),
                _ => return err(ErrorUnauthorized("Could not parse auth header")),
            },
            _ => return err(ErrorUnauthorized("Could not parse auth header")),
        };
        if token.is_none() {
            return err(ErrorUnauthorized("Could not parse auth header"));
        }
        let _token = token.unwrap();

        // let jwk_auth = req.app_data::<Data<JwkAuth>>().expect("Could not get JwkAuth");
        let jwk_auth = req.app_data::<Data<JwkAuth>>().unwrap();
        let token_data = jwk_auth.verify(&_token);
        match token_data {
            Some(data) => ok(RequestUser {
                uid: data.claims.sub,
            }),
            _ => err(ErrorUnauthorized("verification failed")),
        }
    }
}

fn get_token_from_header(header: &str) -> Option<String> {
    let prefix_len = "Bearer ".len();

    match header.len() {
        l if l < prefix_len => None,
        _ => Some(header[prefix_len..].to_string()),
    }
}