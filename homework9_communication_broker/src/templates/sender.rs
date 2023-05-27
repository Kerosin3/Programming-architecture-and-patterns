use serde::{Deserialize, Serialize};
//-------SENDER----------------------------

pub struct SenderContainer(pub Payload);

pub trait SenderExtractor {
    fn transform_to_send(&self) -> Vec<u8>;
}

impl SenderExtractor for SenderContainer {
    fn transform_to_send(&self) -> Vec<u8> {
        serde_json::to_vec(&self.0).unwrap()
    }
}
//------------------------------------------------
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
    pub fn create_play() -> Self {
        OperationObj::Play(OperationAdapter(Box::new(Playgame())))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationAdapter(Box<dyn Operation + Send + Sync + 'static>);

#[typetag::serde(tag = "type")]
trait Operation: std::fmt::Debug + Send + Sync {
    fn extract_operation(&self) {
        println!("extracting something!");
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Playgame();

#[typetag::serde]
impl Operation for Playgame {
    fn extract_operation(&self) {}
}

//https://github.com/EYHN/ddi.git
