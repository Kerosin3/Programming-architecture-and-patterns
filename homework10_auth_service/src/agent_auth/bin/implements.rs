use std::collections::BTreeMap;
use std::default::Default;
use templates::auth::AuthData;
use thiserror::Error;

#[derive(Default, Debug)]
pub(crate) struct Database(BTreeMap<String, AuthData>);

impl Database {
    pub fn insert_to_db(
        &mut self,
        username: &str,
        auth_data: AuthData,
    ) -> Result<(), AuthProcessignError> {
        if self.0.contains_key(username) {
            //error if record exists
            Err(AuthProcessignError::ErrInserting)
        } else {
            println!("recording token and key for user: {}", username);
            self.0.insert(username.to_owned(), auth_data);
            Ok(())
        }
    }
    pub fn test_whether_user_already_registered(&self, username: &str) -> bool {
        self.0.contains_key(username)
    }
    pub fn update_record(
        &mut self,
        username: &str,
        auth_data: AuthData,
    ) -> Result<(), AuthProcessignError> {
        if !self.0.contains_key(username) {
            //error is record not exists
            Err(AuthProcessignError::ErrUpdating)
        } else {
            println!("updating auth tokens for user {}", username);
            self.0.insert(username.to_owned(), auth_data);
            Ok(())
        }
    }
}

#[derive(Copy, Clone, Debug, Error, PartialEq)]
#[non_exhaustive]
pub enum AuthProcessignError {
    #[error("generic error")]
    Error,
    #[error("record exists")]
    ErrInserting,
    #[error("record not exists")]
    ErrUpdating,
}
