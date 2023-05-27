use chrono::prelude::*;
use templates::data_exchange::sender_interface::SenderDataInterface;
use templates::data_exchange::DataContainer;
use templates::data_exchange::OperationObj;

pub struct SenderWrapper(DataContainer);

impl std::default::Default for SenderWrapper {
    fn default() -> Self {
        Self(DataContainer::default())
    }
}

impl SenderDataInterface for SenderWrapper {
    fn transform_to_send(&self) -> Vec<u8> {
        serde_json::to_vec(&self.0).unwrap() // echeck error
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
        self.0.username = name.to_string().to_owned();
        self
    }

    fn assign_arg(mut self, arg: &str) -> Self {
        self.0.args.push(arg.to_string().to_owned());
        self
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
