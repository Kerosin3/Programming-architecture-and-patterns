use jwt_simple::prelude::*;
use std::collections::BTreeMap;
use templates::auth::AuthMessage;

#[derive(Default, Debug)]
pub(crate) struct AuthUsersDB(BTreeMap<String, AuthMessage>);

impl AuthUsersDB {
    pub fn insert_user(&mut self, username: String, auth_data: &AuthMessage) {
        println!("[[[[added user {} to bridge database]]]]", username);
        self.0.insert(username, auth_data.clone());
    }
    pub fn get_key_for_user(&self, username: &str) -> Option<HS256Key> {
        if !self.0.contains_key(username) {
            None
        } else {
            let key_bytes = &self.0.get(username).unwrap().key;
            Some(HS256Key::from_bytes(key_bytes))
        }
    }
}
