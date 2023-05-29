use serde::{Deserialize, Serialize};
use templates::data_exchange::OperationObj;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AgentCommand {
    pub cmd: OperationObj,
    pub arg: Vec<(usize, String)>,
    pub info: AgentInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ServerCommand {
    pub cmd: GameServerCommands,
    pub info: AgentInfo,
    pub args: Vec<(usize, String)>,
}

impl std::convert::From<AgentCommand> for ServerCommand {
    fn from(value: AgentCommand) -> Self {
        Self {
            cmd: value.cmd.into(),
            args: value.arg.to_owned(),
            info: AgentInfo {
                username: value.info.username,
                gameid: value.info.gameid,
                objectid: value.info.objectid,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AgentInfo {
    pub username: String,
    pub gameid: isize,
    pub objectid: isize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum GameServerCommands {
    SrvMoveObject,
    SrvRotateObject,
    SrvTest,
    SrvDbg,
    SrvGameInit,
    SrvPlay,
}

impl std::convert::From<OperationObj> for GameServerCommands {
    fn from(value: OperationObj) -> Self {
        match value {
            OperationObj::InitializeGame => GameServerCommands::SrvGameInit,
            OperationObj::Play => GameServerCommands::SrvPlay,
            OperationObj::Test => GameServerCommands::SrvTest,
            OperationObj::Dgb => GameServerCommands::SrvDbg,
            _ => GameServerCommands::SrvRotateObject,
        }
    }
}
