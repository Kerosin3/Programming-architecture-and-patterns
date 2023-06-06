use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthMessage {
    pub username: String,
    pub rez: Result<Vec<u8>, AuthError>,
    pub token: String,
}

impl AuthMessage {
    pub fn generate_key(&mut self) {
        let key = HS256Key::generate();
        self.rez = Ok(key.to_bytes())
    }
    pub fn generate_token(&mut self) {
        let claims = Claims::create(Duration::from_hours(2));
        let key = self.rez.as_ref().unwrap();
        let token = HS256Key::from_bytes(&key).authenticate(claims).unwrap();
        self.token = token;
    }
}

impl Default for AuthMessage {
    fn default() -> Self {
        Self {
            token: String::default(),
            username: "default_username".to_string(),
            rez: Err(AuthError::NotImplemented),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthError {
    NotAllowed,
    NotImplemented,
}
