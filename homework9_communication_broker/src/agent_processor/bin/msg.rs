use bincode::ErrorKind;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::time::{Duration, SystemTime};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub i: usize,
    pub time: SystemTime,
}

impl From<&Message> for Vec<u8> {
    fn from(value: &Message) -> Self {
        bincode::serialize(value).unwrap()
    }
}

impl From<Message> for Vec<u8> {
    fn from(value: Message) -> Self {
        bincode::serialize(&value).unwrap()
    }
}

impl TryFrom<&[u8]> for Message {
    type Error = Box<ErrorKind>;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        bincode::deserialize(value)
    }
}
