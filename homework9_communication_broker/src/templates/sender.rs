use serde::{Deserialize, Serialize};
//-------SENDER----------------------------

#[derive(Serialize, Deserialize, Debug)]
pub struct SenderDataContainer {
    username: String,
    gameid: isize,
    objectid: isize,
    operation: OperationObj,
}
impl SenderDataContainer {
    pub fn new(
        gameid: isize,
        objectid: isize,
        username: String,
        operation: OperationAdapter,
    ) -> Self {
        Self {
            gameid,
            username,
            objectid,
            operation: OperationObj::create_play(operation),
        }
    }
}
// trait for serializing
pub trait SenderExtractor {
    fn transform_to_send(&self) -> Vec<u8>;
}

impl SenderExtractor for SenderDataContainer {
    fn transform_to_send(&self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap() // echeck error
    }
}
//------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
pub enum OperationObj {
    Auth,
    Play(OperationAdapter),
    Test(String),
}

impl std::default::Default for OperationObj {
    fn default() -> Self {
        Self::Auth
    }
}

impl OperationObj {
    pub fn create_test(str: String) -> Self {
        OperationObj::Test(str)
    }
    pub fn create_play(operadapter: OperationAdapter) -> Self {
        OperationObj::Play(operadapter)
    }
}
//newtype
#[derive(Serialize, Deserialize, Debug)]
pub struct OperationAdapter(pub Box<dyn Operation + Send + Sync + 'static>);

#[typetag::serde(tag = "type")]
pub trait Operation: std::fmt::Debug + Send + Sync {
    fn extract_operation(&self) {
        println!("extracting something!");
    }
}

//https://github.com/EYHN/ddi.git
