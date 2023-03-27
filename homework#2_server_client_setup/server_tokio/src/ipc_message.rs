#![allow(dead_code)]
#![allow(unused_imports)]
use bytes::{Buf, BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IpcMessage {
    pub state: BytesMut,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    TurnOn,
    TurnOff,
    GetProperty,
    MsgBack,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum NetMsgType {
    SendClient,
    SendServer,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub devname: String,
    pub msg_type: NetMsgType,
    pub command: Option<Command>,
    pub info: Option<String>,
    pub errors: Option<String>,
}
impl Message {
    pub fn new(sender: NetMsgType) -> Self {
        Self {
            devname: "default".to_string(),
            msg_type: sender,
            command: None,
            info: None,
            errors: None,
        }
    }
    pub fn assign_command(&mut self, cmd: Command) {
        self.command = Some(cmd);
    }
    pub fn assign_devname(&mut self, devname: String) {
        self.devname = devname;
    }
    pub fn serialize_message(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    pub fn assign_info(&mut self, info: String) {
        self.info = Some(info);
    }
    pub fn deserialize_message(buf: &[u8]) -> Self {
        let out: Message = bincode::deserialize(buf).unwrap();
        out
    }
}
