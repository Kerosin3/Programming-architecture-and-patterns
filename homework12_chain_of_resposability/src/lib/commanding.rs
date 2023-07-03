use std::cell::RefCell;
use std::default::Default;
use std::rc::Rc;
use thiserror::Error;

#[derive(Debug, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum CommandStore {
    MoveCommand,
    RotateCOmmand,
    LogCommand,
    RepeatCommand,
}
// Command trait
pub trait Command {
    fn execute(&self) -> Result<CommandStore, ErrorProcessing>;
    fn get_cmd_name(&self) -> String;
}
// spaceship, etc...
pub trait MainObject: Commandable {
    fn new() -> Self;
    fn init(obj: Self) -> Rc<Self>;
}
//marker
pub trait Commandable: Movable + Rotable {}

//blanket implement
impl<T> Commandable for T where T: Movable + Rotable {}

// movable trait
pub trait Movable {
    type Output;
    fn try_get_position(&self) -> Result<Self::Output, ErrorProcessing>;
}
// rotable trait
pub trait Rotable {
    fn try_rotate_object(&self) -> Result<(), ErrorProcessing>;
}
//--------------------------------------
pub trait Logging {
    fn write_to_log(&mut self, arg: &str);
    fn init_log(&self);
    fn print_log(&self);
    fn test_if_log_is_empty(&self) -> bool;
}
#[derive(Default)]
pub struct Logger(Vec<String>);
impl Logging for Logger {
    fn init_log(&self) {
        println!("LOGGING INITIALIZED!");
    }
    fn write_to_log(&mut self, arg: &str) {
        self.0.push(arg.to_owned());
    }
    fn print_log(&self) {
        for data in self.0.iter() {
            println!(
                "There were errors while executed these commands: [{}]",
                data
            );
        }
    }

    fn test_if_log_is_empty(&self) -> bool {
        if self.0.is_empty() {
            true
        } else {
            false
        }
    }
}

//--------------------------------------

trait CmdProcessor<T: Command> {
    fn process(&self);
}

// commands container
#[derive(Default)]
pub struct Schema {
    commands: Vec<Box<dyn Command>>,
    log: Logger,
}
impl Schema {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_cmd(&mut self, cmd: Box<dyn Command>) {
        self.commands.push(cmd);
    }
    pub fn perform_cmd_single(&mut self) -> Result<(), ErrorProcessing> {
        let cmd = self.commands.pop().unwrap();
        println!("executing cmd");
        cmd.execute()?;
        Ok(())
    }
    pub fn test_log_emptiness(&self) -> bool {
        self.log.test_if_log_is_empty()
    }
    pub fn perform_all(&mut self) -> Result<(), ErrorProcessing> {
        let mut log_inited = false;
        let mut repeat = false;
        for cmd in self.commands.iter() {
            let cur_cmd = cmd.get_cmd_name();
            println!("executing command : {}", &cur_cmd);
            match cmd.execute() {
                // no errors
                Ok(cmd_name) => {
                    if cmd_name == CommandStore::LogCommand && log_inited == false {
                        self.log.init_log(); // init logger
                        log_inited = true;
                    }
                    if cmd_name == CommandStore::RepeatCommand && repeat == false {
                        repeat = true;
                    }
                }
                // process error
                Err(e) => {
                    if log_inited {
                        self.log.write_to_log(&cur_cmd);
                    }
                    if repeat {
                        // execute again on error
                        cmd.execute();
                    }
                    //             return Err(e);
                }
            }
            println!("execution done!");
        }
        Ok(())
    }
    pub fn print_log(&self) {
        self.log.print_log();
    }
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
