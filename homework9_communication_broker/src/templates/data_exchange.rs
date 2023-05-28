use serde::{Deserialize, Serialize};
//-------SENDER----------------------------

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DataContainer {
    pub username: String,
    pub gameid: isize,
    pub objectid: isize,
    pub operation: OperationObj,
    pub args: Vec<String>,
    pub timestamp: String,
    pub dbg: isize,
}

//Sender Interface
pub mod sender_interface {
    use super::OperationObj;
    pub trait SenderDataInterface: Sized {
        fn transform_to_send(&self) -> Vec<u8>;
        fn assign_gameid(self, id: isize) -> Self;
        fn assign_obj_id(self, id: isize) -> Self;
        fn assign_name(self, name: &str) -> Self;
        fn assign_arg(self, arg: &str) -> Self;
        fn assign_operation(self, operation: OperationObj) -> Self;
        fn assign_timestamp(self) -> Self;
        fn assign_dbg(self, dbg: isize) -> Self;
    }
}

//------------------------------------------------
//Receiver Interface
pub mod recv_interface {
    use super::OperationObj;
    pub trait RecvDataInterface: Sized {
        fn get_gameid(&self) -> isize;
        fn get_obj_id(&self) -> isize;
        fn get_name(&self) -> &str;
        fn get_args(&self) -> Vec<String>;
        fn get_operation(&self) -> OperationObj;
        fn get_timestamp(&self) -> String;
        fn get_dbg(&self) -> isize;
    }
}

//------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub enum OperationObj {
    Auth,
    Play,
    Test(String),
    Dgb,
}

impl std::default::Default for OperationObj {
    fn default() -> Self {
        Self::Dgb
    }
}

impl OperationObj {
    pub fn create_test(str: String) -> Self {
        OperationObj::Test(str)
    }
    pub fn create_play() -> Self {
        OperationObj::Play
    }
}
//https://github.com/EYHN/ddi.git
