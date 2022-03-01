use std::collections::HashMap;
use std::future::Future;
use std::sync::Mutex;
use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::Payload, web::Data, Error, FromRequest, HttpRequest, Result, web};
use futures_util::future::{err, ok, Ready};
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use jsonwebtoken::jwk::{AlgorithmParameters, JwkSet};
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserRequest {}

impl FromRequest for UserRequest {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let jwks =  req.app_data::<web::Data<Mutex<JwkSet>>>().unwrap().lock().unwrap();

        if let Some(http_token) = req.headers().get("Authorization") {
            if let Ok(value) = http_token.to_str() {
                debug!("{value:?}");
                if let Some(token) = value.strip_prefix("Bearer ") {
                    if let Ok(header) = decode_header(token) {
                        if let Some(kid) = header.kid {
                            if let Some(j) = jwks.find(&kid) {
                                debug!("we chadding");
                                match j.algorithm {
                                    AlgorithmParameters::RSA(ref rsa) => {
                                        let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).unwrap();
                                        let mut validation = Validation::new(j.common.algorithm.unwrap());
                                        validation.validate_exp = false;
                                        let decoded_token =
                                            decode::<HashMap<String, serde_json::Value>>(token, &decoding_key, &validation)
                                                .unwrap();
                                        debug!("{:?}", decoded_token);
                                        return ok(Self {});
                                    }
                                    _ => unreachable!("this should be a RSA"),
                                }
                            }
                        }
                    }
                    return err(ErrorUnauthorized("Could not verify token"));
                }
            }
            return err(ErrorUnauthorized("Could not parse authorization header"));
        }

        return err(ErrorUnauthorized("Authorization header not set"));
    }
}
