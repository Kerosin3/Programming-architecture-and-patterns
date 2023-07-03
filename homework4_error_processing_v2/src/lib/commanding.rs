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
    fn pre_info(&self) -> CommandStore;
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
        let mut out = String::new();
        for data in self.0.iter() {
            std::fmt::write(&mut out, format_args!("{}, ", data))
                .expect("Error occurred while trying to write in String");
        }
        println!("There were errors while executed these commands: [{}]", out);
    }

    fn test_if_log_is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

//--------------------------------------

pub trait CmdProcessor: Default {
    fn process(
        &self,
        rez: Result<CommandStore, ErrorProcessing>,
        logger: &mut dyn Logging,
        log_en: bool,
        cmd_name: &str,
    ) {
        match rez {
            Ok(_) => {}
            Err(e) => {
                if log_en {
                    logger.write_to_log(cmd_name);
                    logger.write_to_log(e.to_string().as_str());
                }
            }
        }
    }
}
#[derive(Default)]
pub struct Processor {}

impl CmdProcessor for Processor {}
//--------------------------------------
// commands container
#[derive(Default)]
pub struct Schema<T: CmdProcessor> {
    commands: Vec<Box<dyn Command>>,
    log: Logger,
    processor: T,
    log_enabled: bool,
    repeater: bool,
}
impl<T: CmdProcessor> Schema<T> {
    pub fn new() -> Self {
        Self {
            commands: Vec::default(),
            log: Logger::default(),
            processor: T::default(),
            log_enabled: false,
            repeater: false,
        }
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
        for cmd in self.commands.iter_mut() {
            let cur_cmd = cmd.get_cmd_name();
            let cur_cmd_enum = cmd.pre_info();
            //enable log
            if (!self.log_enabled) && cur_cmd_enum == CommandStore::LogCommand {
                self.log_enabled = true;
                self.log.init_log();
            }
            if (!self.repeater) && cur_cmd_enum == CommandStore::RepeatCommand {
                self.repeater = true;
            }
            let execution_result = cmd.execute();
            //repeat execution
            if (self.repeater) && execution_result.is_err() {
                let r = cmd.execute();
                self.processor
                    .process(r, &mut self.log, self.log_enabled, &cur_cmd);
                self.log_enabled = false;
            }
            //write log if it is neccessary
            self.processor
                .process(execution_result, &mut self.log, self.log_enabled, &cur_cmd);
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
