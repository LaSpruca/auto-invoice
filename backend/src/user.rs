use actix_web::{dev::Payload, Error, FromRequest, HttpRequest, Result, web};
use actix_web::error::ErrorUnauthorized;
use chrono::{Utc};
use futures_util::future::{err, ok, Ready};
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use jsonwebtoken::jwk::{AlgorithmParameters};
use serde::{Deserialize, Serialize};

use crate::AuthZState;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserRequest {
    exp: i64,
    iat: i64,
    sub: String,
    iss: String,
    aud: Vec<String>,
}

impl FromRequest for UserRequest {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let authz_state = req.app_data::<web::Data<AuthZState>>().unwrap();
        let jwks = authz_state.tokens.lock().unwrap();

        if let Some(http_token) = req.headers().get("Authorization") {
            if let Ok(value) = http_token.to_str() {
                if let Some(token) = value.strip_prefix("Bearer ") {
                    if let Ok(header) = decode_header(token) {
                        if let Some(kid) = header.kid {
                            if let Some(j) = jwks.find(&kid) {
                                match j.algorithm {
                                    AlgorithmParameters::RSA(ref rsa) => {
                                        let decoding_key =
                                            DecodingKey::from_rsa_components(&rsa.n, &rsa.e)
                                                .unwrap();
                                        let mut validation =
                                            Validation::new(j.common.algorithm.unwrap());
                                        validation.validate_exp = false;
                                        let decoded_token =
                                            decode::<Self>(token, &decoding_key, &validation)
                                                .unwrap();

                                        let claims = decoded_token.claims.clone();

                                        let now = Utc::now().timestamp();

                                        let aud = vec![
                                            authz_state.audience.clone(),
                                            authz_state.authority.clone(),
                                        ];

                                        if now < claims.exp {
                                            if claims.aud.iter().fold(true, |flag, current| {
                                                if flag {
                                                    aud.contains(current)
                                                } else {
                                                    false
                                                }
                                            }) {
                                                return ok(decoded_token.claims);
                                            }

                                            return err(ErrorUnauthorized("Invalid audience"));
                                        }

                                        return err(ErrorUnauthorized("Token expired"));
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
