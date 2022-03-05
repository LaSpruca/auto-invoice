use jsonwebtoken::jwk::JwkSet;
use std::sync::{Arc, Mutex};

pub struct AuthZState {
    pub tokens: Arc<Mutex<JwkSet>>,
    pub audience: String,
    pub authority: String,
}
