#[allow(unreachable_patterns)]

pub mod commanding {
    use std::rc::Rc;

    use crate::obj_mech::object_moving::*;
    pub struct Schema<'a> {
        commands: Vec<&'a dyn Command>,
        log_adapt: Option<LogAdapter>,
    }

    impl<'t> Default for Schema<'t> {
        fn default() -> Self {
            Self {
                commands: vec![],
                log_adapt: None,
            }
        }
    }

    impl<'t> Schema<'t> {
        pub fn new() -> Self {
            Self {
                commands: vec![],
                log_adapt: None,
            }
        }
        pub fn assign_logger(&mut self, logger: Rc<slog::Logger>) {
            self.log_adapt = Some(LogAdapter { logger: logger });
        }
        pub fn push(&mut self, cmd: &'t dyn Command) {
            self.commands.push(cmd);
        }
        pub fn execute(
            &self,
            obj: &mut dyn Commandable<Coordinates = Point2D<i32>>,
        ) -> Result<(), ErrorProcessing> {
            for cmd in self.commands.iter() {
                if let Err(e) = cmd.perform(obj) {
                    // if log activated -> write to log
                    if let Some(lg) = &self.log_adapt {
                        lg.process_error(e)
                    } else {
                        println!("LOGIS NOT ACTIVATED!");
                    }
                }
            }
            Ok(())
        }
    }
    pub struct LogAdapter {
        logger: Rc<slog::Logger>,
    }

    pub trait ErrorHandler {
        fn process_error(&self, err: ErrorProcessing);
    }
    impl ErrorHandler for LogAdapter {
        fn process_error(&self, err: ErrorProcessing) {
            slog::info!(*self.logger , "error happened!"; "error:" => err.to_string() );
        }
    }
    // trait for custom purposes
    pub trait GeneralPurpose {
        fn enable_logging(&mut self) {}
    }

    pub trait Command {
        fn perform(
            &self,
            obj: &mut dyn Commandable<Coordinates = Point2D<i32>>,
        ) -> Result<(), ErrorProcessing>;
    }
    //------------------------------------------------------------------
    //------------------------------------------------------------------
    //------------------------------------------------------------------
    //blanket implement
    impl<T> Commandable for T where T: MoveObj + RotateObj + Consumable + GeneralPurpose + ?Sized {}
    //------------------------------------------------------------------
    //------------------------------------------------------------------
    //------------------------------------------------------------------
    #[non_exhaustive]
    pub enum CommandStore {
        WriteLog, //enable write to log command
        MoveCommand,
        RotateCommand,
        CheckFuel,
        BurnFuel,
        RotateWith,
        SomeErrorPerform,
    }
    impl Command for CommandStore {
        fn perform(
            &self,
            obj: &mut dyn Commandable<Coordinates = Point2D<i32>>,
        ) -> Result<(), ErrorProcessing> {
            match self {
                CommandStore::WriteLog => obj.enable_logging(),
                CommandStore::MoveCommand => obj.execute_move()?,
                CommandStore::RotateCommand => obj.execute_rotate()?,
                CommandStore::CheckFuel => obj.check()?,
                CommandStore::BurnFuel => obj.consume(),
                _ => return Err(ErrorProcessing::Err),
            }
            Ok(())
        }
    }
}
