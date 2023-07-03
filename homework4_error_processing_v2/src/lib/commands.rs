pub mod examples {
    use super::super::commanding::*;
    use std::rc::Rc;
    // Move command
    //--------------------------------------
    pub struct MoveCommand<T: Commandable> {
        obj: Rc<T>,
    }

    impl<T: Commandable> MoveCommand<T> {
        pub fn new(obj: Rc<T>) -> Self {
            Self { obj }
        }
    }

    impl<T: Commandable> Command for MoveCommand<T> {
        fn execute(&self) -> Result<CommandStore, ErrorProcessing> {
            println!("executing command [[{}]]", self.get_cmd_name());
            self.obj.try_get_position()?;
            Ok(CommandStore::MoveCommand)
        }

        fn get_cmd_name(&self) -> String {
            "MoveCommand".to_string()
        }

        fn pre_info(&self) -> CommandStore {
            CommandStore::MoveCommand
        }
    }
    //--------------------------------------
    // Rotate command
    pub struct RotateCommand<T: Commandable> {
        obj: Rc<T>,
    }
    impl<T: Commandable> RotateCommand<T> {
        pub fn new(obj: Rc<T>) -> Self {
            Self { obj }
        }
    }
    impl<T: Commandable> Command for RotateCommand<T> {
        fn execute(&self) -> Result<CommandStore, ErrorProcessing> {
            println!("executing command [[{}]]", self.get_cmd_name());
            self.obj.try_rotate_object()?;
            Ok(CommandStore::RotateCOmmand)
        }

        fn get_cmd_name(&self) -> String {
            "RotateCommand".to_string()
        }

        fn pre_info(&self) -> CommandStore {
            CommandStore::RotateCOmmand
        }
    }
    //--------------------------------------
    pub struct LogCommand<T: Commandable> {
        _obj: Rc<T>,
    }
    impl<T: Commandable> LogCommand<T> {
        pub fn new(obj: Rc<T>) -> Self {
            Self { _obj: obj }
        }
    }
    impl<T: Commandable> Command for LogCommand<T> {
        fn execute(&self) -> Result<CommandStore, ErrorProcessing> {
            Ok(CommandStore::LogCommand)
        }

        fn get_cmd_name(&self) -> String {
            "LogCommand".to_string()
        }

        fn pre_info(&self) -> CommandStore {
            CommandStore::LogCommand
        }
    }
    //--------------------------------------
    pub struct RepeatCommand<T: Commandable> {
        _obj: Rc<T>,
    }
    impl<T: Commandable> RepeatCommand<T> {
        pub fn new(obj: Rc<T>) -> Self {
            Self { _obj: obj }
        }
    }
    impl<T: Commandable> Command for RepeatCommand<T> {
        fn execute(&self) -> Result<CommandStore, ErrorProcessing> {
            println!("executing command [[{}]]", self.get_cmd_name());
            Ok(CommandStore::RepeatCommand)
        }

        fn get_cmd_name(&self) -> String {
            "RepeatCommand".to_string()
        }

        fn pre_info(&self) -> CommandStore {
            CommandStore::RepeatCommand
        }
    }
}
