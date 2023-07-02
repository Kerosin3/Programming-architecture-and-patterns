use std::cell::RefCell;
use std::default::Default;
use std::rc::Rc;
use thiserror::Error;
// for generic main object

pub trait MainObject {
    fn init() -> Self;
    fn setup(&mut self);
}

// adapter for schema

pub struct Schema<J: MainObject + Default> {
    commands: Vec<Box<dyn Commandable>>,
    container: J,
}

pub trait CommandFactory {
    fn emit_command(cmd: &CommandStore) -> Box<dyn Commandable>;
}

pub struct CmdFactory {}
impl CommandFactory for CmdFactory {
    fn emit_command(cmd: &CommandStore) -> Box<dyn Commandable> {
        match cmd {
            CommandStore::DefaultCommand => Box::new(),
            CommandStore::MoveCommand => Box::new(),
        }
    }
}

pub trait Commandable {}

//blanket
impl<T> Commandable for T where T: Movable + Rotable {}

#[non_exhaustive]
pub enum CommandStore {
    DefaultCommand,
    MoveCommand,
}

pub trait Movable {
    type Adapter = MainObject;
    fn try_get_position(&self);
}
pub trait Rotable {
    fn try_rotate_object(&self);
}

#[derive(Copy, Clone, Debug, Error, PartialEq)]
#[non_exhaustive]
pub enum ErrorProcessing {
    #[error("generic error")]
    Err,
    #[error("error getting info")]
    ErrGetInfo,
    #[error("error moving")]
    ErrMoving,
    #[error("error rotating")]
    ErrRotating,
    #[error("error setting")]
    ErrSetting,
    #[error("error checking")]
    ErrCheck,
    #[error("error consuming")]
    ErrConsum,
}
