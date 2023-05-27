use rumqttc::Publish;
use templates::data_exchange::recv_interface::RecvDataInterface;
use templates::data_exchange::DataContainer;
use templates::data_exchange::OperationObj;

pub struct RecvWrapper(DataContainer);
impl std::default::Default for RecvWrapper {
    fn default() -> Self {
        Self(DataContainer::default())
    }
}
impl RecvWrapper {
    pub fn deserialize_data(data: Publish) -> Result<Self, serde_json::Error> {
        let recv_data: Result<DataContainer, serde_json::Error> =
            serde_json::from_slice(&data.payload.to_vec());
        match recv_data {
            Ok(d) => Ok(Self(d)),
            Err(e) => Err(e),
        }
    }
}
impl RecvDataInterface for RecvWrapper {
    fn get_gameid(&mut self) -> isize {
        self.0.gameid
    }

    fn get_obj_id(&mut self) -> isize {
        self.0.objectid
    }

    fn get_name(&mut self) -> &str {
        &self.0.username
    }

    fn get_args(&mut self) -> Vec<String> {
        self.0.args.to_owned()
    }

    fn get_operation(&mut self) -> OperationObj {
        self.0.operation.to_owned()
    }

    fn get_timestamp(&self) -> String {
        self.0.timestamp.to_owned()
    }

    fn get_dbg(&self) -> isize {
        self.0.dbg
    }
}

impl std::fmt::Display for RecvWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "username: {}\ngameid:{},object id {}\noperation {:?}\ntimestamp:{}\ndebug:{}\nARGS:[{:?}]",
            self.0.username,
            self.0.gameid,
            self.0.objectid,
            self.0.operation,
            self.0.timestamp,
            self.0.dbg,
            self.0.args
        )
    }
}
