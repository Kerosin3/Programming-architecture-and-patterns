use super::data_exchange::OperationObj;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AgentInfo {
    pub username: String,
    pub gameid: isize,
    pub objectid: isize,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ServerCommand {
    pub cmd: GameServerCommands,
    pub info: AgentInfo,
    pub args: Vec<(usize, String)>,
}
impl ServerCommand {
    pub fn get_username(&self) -> String {
        self.info.username.to_owned()
    }
    pub fn get_args(&self) -> Vec<String> {
        let mut out = vec![];
        for s in self.args.iter() {
            out.push(s.1.to_owned())
        }
        out
    }
}
impl std::fmt::Display for ServerCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "agentname: {}, command :{:?}, args: {:?}",
            self.info.username, self.cmd, self.args
        )
    }
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AgentCommand {
    pub cmd: OperationObj,
    pub arg: Vec<(usize, String)>,
    pub info: AgentInfo,
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