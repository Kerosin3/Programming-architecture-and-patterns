use std::collections::BTreeMap;
use templates::auth::AuthMessage;

#[derive(Default, Debug)]
pub(crate) struct AuthUsersDB(BTreeMap<String, AuthMessage>);

impl AuthUsersDB {
    pub fn insert_user(&mut self, username: String, auth_data: &AuthMessage) {
        println!("added user {} to bridge database", username);
        self.0.insert(username, auth_data.clone());
    }
}
