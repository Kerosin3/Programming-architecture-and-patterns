use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Mydata<T: OperationConstructor + OperationSender> {
    pub operation: T, // auth, play so on...
}

pub trait OperationConstructor {
    fn new(&self) -> OperationObj;
}
pub trait OperationSender {
    fn metamorphose(&self) -> Vec<u8>;
}

pub struct SenderEndpointAdapter {
    data: Vec<u8>,
}
pub struct ReceiverEndpointAdapter {
    data: Vec<u8>,
}

/*
pub trait OperationExecutor {
    fn perform(&self) -> Result<()>;
}
pub trait OperationSender: OperationProto + OperationConstructor {} // marker trait

pub trait OperationProto {
    fn send_command(&self, operation: Box<dyn OperationExecutor>) -> Vec<u8>;
}

*/
#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum OperationObj {
    Auth,
    Play,
    Test(String),
}
