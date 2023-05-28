use anyhow::Result;
use num::Num;
use rumqttc::Publish;
use std::collections::BTreeMap;
use std::default::Default;
use std::fmt::Debug;
use templates::args::Argument;
use templates::data_exchange::recv_interface::ErrorR;
use templates::data_exchange::recv_interface::RecvDataInterface;
use templates::data_exchange::DataContainer;
use templates::data_exchange::OperationObj;

pub struct RecvWrapper<U: Num + Copy + Default>(DataContainer<U>);
impl<U: Default + Num + Copy> Default for RecvWrapper<U> {
    fn default() -> Self {
        Self(DataContainer::default())
    }
}

impl<T: Default + Num + serde::de::DeserializeOwned + Copy> RecvWrapper<T> {
    pub fn deserialize_data(data: &Publish) -> Result<Self, serde_json::Error> {
        let recv_data: Result<DataContainer<T>, serde_json::Error> =
            serde_json::from_slice(&data.payload.to_vec());
        match recv_data {
            Ok(d) => Ok(Self(d)),
            Err(e) => Err(e),
        }
    }
}
impl<T: Default + Num + Debug + Copy> RecvDataInterface<T> for RecvWrapper<T> {
    fn get_gameid(&self) -> isize {
        self.0.gameid
    }

    fn get_obj_id(&self) -> isize {
        self.0.objectid
    }

    fn get_name(&self) -> &str {
        &self.0.username
    }

    fn get_operation(&self) -> OperationObj {
        self.0.operation.to_owned()
    }

    fn get_timestamp(&self) -> String {
        self.0.timestamp.to_owned()
    }

    fn get_dbg(&self) -> isize {
        self.0.dbg
    }

    fn get_args(&self, id: usize) -> Result<(T, String), ErrorR> {
        //         self.0.args.get(&id).ok_or_else(|| ErrorR::ErrorArg)
        let Some(arg)  =self.0.args.get(&id) else {
            return Err(ErrorR::ErrorArg)
        };
        Ok((
            arg.try_get_num().ok_or(ErrorR::EmptyVariant)?,
            arg.try_get_string().ok_or(ErrorR::EmptyVariant)?,
        ))
    }

    fn get_all_args_pairs(&self) -> Result<Vec<(usize, String)>, ErrorR> {
        let mut out = vec![];
        for (k, v) in self.0.args.iter() {
            out.push((*k, v.try_get_string().ok_or(ErrorR::EmptyVariant)?))
        }
        if out.is_empty() {
            return Err(ErrorR::EmptyVariant);
        }
        Ok(out)
    }
}

impl<U: Default + Num + Debug + Copy> std::fmt::Display for RecvWrapper<U> {
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
