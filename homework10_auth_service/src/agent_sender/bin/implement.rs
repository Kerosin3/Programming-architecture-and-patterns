use chrono::prelude::*;
use templates::args::Argument;
use templates::data_exchange::sender_interface::ErrorS;
use templates::data_exchange::sender_interface::SenderDataInterface;
use templates::data_exchange::DataContainer;
use templates::data_exchange::OperationObj;
pub struct SenderWrapper<Z: num::Num + std::default::Default + Copy>(DataContainer<Z>);

impl<T: num::Num + std::default::Default + Copy> std::default::Default for SenderWrapper<T> {
    fn default() -> Self {
        Self(DataContainer::default())
    }
}

impl std::fmt::Display for SenderWrapper<i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "username:{}, gameid:{}, operation:{:?}, obj_id:{} timestampl:{}",
            self.0.username, self.0.gameid, self.0.operation, self.0.objectid, self.0.timestamp
        )
    }
}

impl<U: num::Num + serde::Serialize + std::default::Default + Copy> SenderDataInterface<U>
    for SenderWrapper<U>
{
    fn transform_to_send(&self) -> Vec<u8> {
        serde_json::to_vec(&self.0).unwrap() // check error!
    }

    fn assign_gameid(mut self, id: isize) -> Self {
        self.0.gameid = id;
        self
    }

    fn assign_obj_id(mut self, id: isize) -> Self {
        self.0.objectid = id;
        self
    }

    fn assign_name(mut self, name: &str) -> Self {
        self.0.username = name.to_string();
        self
    }
    fn assign_arg(mut self, arg_id: usize, arg: Argument<U>) -> Result<Self, ErrorS> {
        match self.0.args.insert(arg_id, arg.finallize()) {
            None => Ok(self),
            Some(_) => Err(ErrorS::ErrorArg),
        }
    }

    fn assign_operation(mut self, operation: OperationObj) -> Self {
        self.0.operation = operation;
        self
    }

    fn assign_timestamp(mut self) -> Self {
        let utc: DateTime<Utc> = Utc::now();
        self.0.timestamp = utc.to_string();
        self
    }

    fn assign_dbg(mut self, dbg: isize) -> Self {
        self.0.dbg = dbg;
        self
    }
}