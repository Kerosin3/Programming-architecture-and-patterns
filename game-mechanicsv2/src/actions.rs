pub mod Commanding {
    use crate::obj_mech::object_moving::*;
    pub struct Schema<'a> {
        commands: Vec<&'a dyn Command>,
    }
    impl<'t> Schema<'t> {
        pub fn new() -> Self {
            Self { commands: vec![] }
        }
        pub fn push(&mut self, cmd: &'t dyn Command) {
            self.commands.push(cmd);
        }
        pub fn execute(
            &self,
            obj: &mut dyn Commandable<Coordinates = Point2D<i32>>,
        ) -> Result<(), ErrorProcessing> {
            for cmd in self.commands.iter() {
                cmd.perform(obj)?;
            }
            Ok(())
        }
    }
    pub trait Command {
        fn perform(
            &self,
            obj: &mut dyn Commandable<Coordinates = Point2D<i32>>,
        ) -> Result<(), ErrorProcessing>;
        //         fn perform_action(&mut self);
        //         fn finallize(&self);
    }
    //------------------------------------------------------------------
    //------------------------------------------------------------------
    //------------------------------------------------------------------
    //blanket implement
    impl<T> Commandable for T where T: MoveObj + RotateObj + ?Sized {}
    //------------------------------------------------------------------
    //------------------------------------------------------------------
    //------------------------------------------------------------------
    #[non_exhaustive]
    pub enum CommandStore {
        MoveCommand,
        RotateCommand,
        #[doc(hidden)]
        __Nonexhaustive,
    }
    impl Command for CommandStore {
        fn perform(
            &self,
            obj: &mut dyn Commandable<Coordinates = Point2D<i32>>,
        ) -> Result<(), ErrorProcessing> {
            match self {
                CommandStore::MoveCommand => obj.execute_move()?,
                CommandStore::RotateCommand => obj.execute_rotate()?,
                _ => return Err(ErrorProcessing::Err),
            }
            Ok(())
        }
    }
}
