use super::args::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::default::Default;
//-------SENDER----------------------------

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DataContainer<X: num::Num + std::default::Default + Copy> {
    pub username: String,
    pub gameid: isize,
    pub objectid: isize,
    // operation to send to processor
    pub operation: OperationObj,
    // store args
    pub args: BTreeMap<usize, Argument<X>>,
    pub timestamp: String,
    pub dbg: isize,
}

//Sender Interface
pub mod sender_interface {
    use super::Argument;
    use super::OperationObj;
    use anyhow::Result;
    use num::Num;
    use std::default::Default;

    pub trait SenderDataInterface<X>: Sized
    where
        X: Num + Default + Copy,
    {
        fn transform_to_send(&self) -> Vec<u8>;
        fn assign_gameid(self, id: isize) -> Self;
        fn assign_obj_id(self, id: isize) -> Self;
        fn assign_name(self, name: &str) -> Self;
        fn assign_arg(self, arg_id: usize, arg: Argument<X>) -> Result<Self, ErrorS>;
        fn assign_operation(self, operation: OperationObj) -> Self;
        fn assign_timestamp(self) -> Self;
        fn assign_dbg(self, dbg: isize) -> Self;
    }
    #[derive(thiserror::Error, Debug, Clone)]
    #[non_exhaustive]
    pub enum ErrorS {
        #[error("Internal error.")]
        Internal(String),
        #[error("Error setting arg")]
        ErrorArg,
    }
}

//------------------------------------------------
//Receiver Interface
pub mod recv_interface {

    use super::OperationObj;
    use num::Num;
    use std::default::Default;

    pub trait RecvDataInterface<X>: Sized
    where
        X: Default + Num,
    {
        fn get_gameid(&self) -> isize;
        fn get_obj_id(&self) -> isize;
        fn get_name(&self) -> &str;
        fn get_args(&self, id: usize) -> Result<(X, String), ErrorR>;
        fn get_operation(&self) -> OperationObj;
        fn get_timestamp(&self) -> String;
        fn get_dbg(&self) -> isize;
        fn get_all_args_pairs(&self) -> Result<Vec<(usize, String)>, ErrorR>;
    }

    #[derive(thiserror::Error, Debug, Clone)]
    #[non_exhaustive]
    pub enum ErrorR {
        #[error("Internal error.")]
        Internal(String),
        #[error("Error getting arg")]
        ErrorArg,
        #[error("Empty variang")]
        EmptyVariant,
        #[error("No variants")]
        NoVariants,
    }
}

//------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Copy)]
#[non_exhaustive]
pub enum OperationObj {
    //     Auth,
    InitializeGame,
    Play,
    Test,
    Dgb,
}

impl std::default::Default for OperationObj {
    fn default() -> Self {
        Self::Dgb
    }
}

impl OperationObj {
    pub fn create_test(_str: String) -> Self {
        OperationObj::Test
    }
    pub fn create_play() -> Self {
        OperationObj::Play
    }
}
//https://github.com/EYHN/ddi.git
