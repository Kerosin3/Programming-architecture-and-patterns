use libcommand::commanding::*;
use libcommand::commands::examples::LogCommand;
use libcommand::commands::examples::MoveCommand;
use libcommand::commands::examples::RepeatCommand;
use libcommand::commands::examples::RotateCommand;
use std::rc::Rc;

#[cfg(test)]
mod ok_setup {
    use super::*;
    #[derive(Default)]
    pub struct SpaceShip {
        coord: i32,
        vel: i32,
    }

    impl MainObject for SpaceShip {
        fn new() -> Self {
            SpaceShip::default()
        }

        fn init(obj: Self) -> Rc<Self> {
            Rc::new(obj)
        }
    }

    impl Movable for SpaceShip {
        type Output = i32;

        fn try_get_position(&self) -> Result<Self::Output, ErrorProcessing> {
            Ok(self.coord)
        }
    }
    impl Rotable for SpaceShip {
        fn try_rotate_object(&self) -> Result<(), ErrorProcessing> {
            self.vel;
            Ok(())
        }
    }
}
//Реализовать Команду, которая записывает информацию о выброшенном исключении в лог.
//Реализовать обработчик исключения, который ставит Команду, пишущую в лог в очередь Команд.
#[test]
fn test_log_init_and_write() {
    use ok_setup::SpaceShip;
    let spaceship = SpaceShip::new();
    let ps = SpaceShip::init(spaceship);
    let lcommand = LogCommand::new(Rc::clone(&ps));
    let mcommand = MoveCommand::new(Rc::clone(&ps));
    let rcommand = RotateCommand::new(Rc::clone(&ps));
    let mut schema = Schema::<Processor>::new();
    schema.push_cmd(Box::new(lcommand));
    schema.push_cmd(Box::new(mcommand));
    schema.push_cmd(Box::new(rcommand));
    assert!(schema.perform_all().is_ok());
    schema.print_log();
    assert!(schema.test_log_emptiness()); // no errors -> empty log
    println!("NO ERRORS -> EMPTY LOG!");
}
#[test]
fn test_repeat_command_no_errors() {
    use ok_setup::SpaceShip;
    let spaceship = SpaceShip::new();
    let ps = SpaceShip::init(spaceship);
    let lcommand = LogCommand::new(Rc::clone(&ps));
    let mcommand = MoveCommand::new(Rc::clone(&ps));
    let rcommand = RotateCommand::new(Rc::clone(&ps));
    let repeat = RepeatCommand::new(Rc::clone(&ps));
    let mut schema = Schema::<Processor>::new();
    schema.push_cmd(Box::new(lcommand));
    schema.push_cmd(Box::new(repeat));
    schema.push_cmd(Box::new(mcommand));
    schema.push_cmd(Box::new(rcommand));
    assert!(schema.perform_all().is_ok());
    schema.print_log();
    assert!(schema.test_log_emptiness()); // no errors - no repeats
    println!("REPEATS ON, NO ERRORS -> EMPTY LOG -> NO REPEATS!");
}
mod err_setup {
    use super::*;
    #[derive(Default)]
    pub struct SpaceShip {
        coord: i32,
        vel: i32,
    }

    impl MainObject for SpaceShip {
        fn new() -> Self {
            SpaceShip::default()
        }

        fn init(obj: Self) -> Rc<Self> {
            Rc::new(obj)
        }
    }

    impl Movable for SpaceShip {
        type Output = i32;

        fn try_get_position(&self) -> Result<Self::Output, ErrorProcessing> {
            Err(ErrorProcessing::ErrGetInfo)
        }
    }
    impl Rotable for SpaceShip {
        fn try_rotate_object(&self) -> Result<(), ErrorProcessing> {
            self.vel;
            Ok(())
        }
    }
    #[test]
    fn test_err_process() {
        use err_setup::SpaceShip;
        let spaceship = SpaceShip::new();
        let ps = SpaceShip::init(spaceship);
        let lcommand = LogCommand::new(Rc::clone(&ps));
        let mcommand = MoveCommand::new(Rc::clone(&ps));
        let rcommand = RotateCommand::new(Rc::clone(&ps));
        let repeat = RepeatCommand::new(Rc::clone(&ps));
        let mut schema = Schema::<Processor>::new();
        schema.push_cmd(Box::new(lcommand));
        schema.push_cmd(Box::new(repeat));
        schema.push_cmd(Box::new(mcommand));
        schema.push_cmd(Box::new(rcommand));
        assert!(schema.perform_all().is_ok());
        schema.print_log();
        assert!(!schema.test_log_emptiness()); // erros -> log is not empty, repeats are working
        println!("ERRORS -> NOT EMPTY LOG, REPEATS ON!");
    }
}
