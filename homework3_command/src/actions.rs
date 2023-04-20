#[allow(unreachable_patterns)]

pub mod commanding {
    use crate::obj_mech::object_moving::*;

    pub struct Schema<'a> {
        commands: Vec<&'a dyn Command>,
    }
    impl<'t> Default for Schema<'t> {
        fn default() -> Self {
            Self::new()
        }
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
        pub fn move_and_consume_fuel() -> Result<Self, ErrorProcessing> {
            let mut shema = Schema::new();
            shema.push(&CommandStore::CheckFuel);
            shema.push(&CommandStore::MoveCommand);
            shema.push(&CommandStore::BurnFuel);
            Ok(shema)
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
    impl<T> Commandable for T where T: MoveObj + RotateObj + Consumable + ?Sized {}
    //------------------------------------------------------------------
    //------------------------------------------------------------------
    //------------------------------------------------------------------
    #[non_exhaustive]
    pub enum CommandStore {
        MoveCommand,
        RotateCommand,
        CheckFuel,
        BurnFuel,
        RotateWith,
    }
    impl Command for CommandStore {
        fn perform(
            &self,
            obj: &mut dyn Commandable<Coordinates = Point2D<i32>>,
        ) -> Result<(), ErrorProcessing> {
            match self {
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
