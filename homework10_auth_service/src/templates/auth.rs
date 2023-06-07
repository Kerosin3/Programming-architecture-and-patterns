use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthMessage {
    pub username: String,
    pub key: Vec<u8>,
    pub token: String,
    pub status: AuthError,
}

impl AuthMessage {
    pub fn get_restored_key(&self) -> HS256Key {
        HS256Key::from_bytes(&self.key)
    }
}

pub trait GeneratorAuth {
    fn gen_key(&mut self);
    fn gen_token(&mut self);
    fn get_auth_data_copy(&self) -> AuthData;
    fn get_serialized(&self) -> Vec<u8>;
    fn set_username(&mut self, name: String);
    fn assign_gameid(&mut self, id: isize);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthData {
    pub uname: String,
    pub key: Vec<u8>,
    pub token: String,
    pub gameid: isize,
}
impl AuthData {
    fn get_key_form(&self) -> HS256Key {
        HS256Key::from_bytes(&self.key)
    }
}
// trait DegeneratorAuth{}

impl AuthMessage {
    pub fn generate_key(&mut self) {
        self.key = HS256Key::generate().to_bytes();
    }
    pub fn generate_token(&mut self) {
        let claims = Claims::create(Duration::from_hours(2));
        let key = self.key.as_ref();
        let token = HS256Key::from_bytes(&key).authenticate(claims).unwrap();
        self.token = token;
    }
    pub fn assign_status_ok(&mut self, id: isize) {
        self.status = AuthError::Okey(id)
    }
}

impl Default for AuthMessage {
    fn default() -> Self {
        Self {
            token: String::default(),
            username: "default_username".to_string(),
            key: vec![],
            status: AuthError::NotImplemented,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Eq)]
pub enum AuthError {
    NotAllowed,
    NotImplemented,
    Okey(isize),
}
