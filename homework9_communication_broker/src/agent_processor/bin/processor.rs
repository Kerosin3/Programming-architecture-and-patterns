use serde::{Deserialize, Serialize};
use templates::data_exchange::OperationObj;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ServerCommand {
    pub cmd: OperationObj,
    pub arg: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[non_exhaustive]
pub enum GameServerCommands {
    MoveObject,
    RotateObject,
    Test,
    Dbg,
}

impl GameServerCommands {
    pub fn command_parser(cmd_to_process: OperationObj) -> Self {
        match cmd_to_process {
            OperationObj::Play => GameServerCommands::MoveObject,
            OperationObj::Test => GameServerCommands::RotateObject,
            OperationObj::Dgb => GameServerCommands::MoveObject,
            _ => todo!(),
        }
    }
}
