use serde::{Deserialize, Serialize};
//-------SENDER----------------------------
pub trait SenderExtractor {
    fn transform_to_send(&self) -> Vec<u8>;
}

impl SenderExtractor for SenderContainer {
    fn transform_to_send(&self) -> Vec<u8> {
        serde_json::to_vec(&self.0).unwrap()
    }
}

pub struct SenderContainer(pub Payload);

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    username: String,
    operation: OperationObj, // id of operation
}

impl Payload {
    pub fn new(username: String, arg: OperationObj) -> Self {
        Self {
            username,
            operation: arg,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
pub enum OperationObj {
    Auth,
    Play,
    Test(String),
}
//https://github.com/EYHN/ddi.git
