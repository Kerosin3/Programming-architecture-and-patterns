#[cfg(test)]
#[allow(unused_imports)]
mod oject_interface_test {

    use lib_game::actions::commanding::*;
    use lib_game::obj_mech::object_moving::*;
    use mockall::predicate::*;
    use mockall::*;
    use rstest::rstest;
    #[cfg(test)]
    mod test_moving_object {
        use super::*;
        use mockall::predicate::*;
        use mockall::*;
        use rstest::rstest;
        use slog::Drain;
        use std::convert::{From, Into};
        use std::rc::Rc;
        use std::sync::Mutex;
        //------------------------------------------------------
        //------------------------------------------------------
        //example class
        #[derive(Clone, Debug)]
        pub struct SpaceShip<T> {
            coord: Point2D<T>,
            velocity: Point2D<T>,
            direction_numbers: i8,
            angular_velocity: i8,
            direction: i8,
            fuel: Fuel,
            pub logger_enabled: bool,
            pub logger: Rc<slog::Logger>,
        }
        impl SpaceShip<i32> {
            pub fn new() -> Self {
                let dp = Point2D::default();
                let decorator = slog_term::TermDecorator::new().build();
                let drain = Mutex::new(slog_term::FullFormat::new(decorator).build()).fuse();
                let log = Rc::new(slog::Logger::root(drain, slog::o!()));

                Self {
                    coord: dp,
                    velocity: dp,
                    direction_numbers: DIRECTION_NUMBERS,
                    angular_velocity: 0,
                    direction: 2,
                    fuel: Fuel::default(),
                    logger_enabled: false,
                    logger: log,
                }
            }
            pub fn set_initial(&mut self, pos: Point2D<i32>, vel: Point2D<i32>) {
                self.coord = pos;
                self.velocity = vel;
            }
            pub fn set_initial_angular(&mut self, mut angular_data: i8) {
                if angular_data.abs() > 7 {
                    angular_data = 0;
                }
                self.angular_velocity = angular_data;
            }
            pub fn set_initial_direction(&mut self, mut direction: i8) {
                if direction.abs() > 7 {
                    direction = 0;
                }
                self.direction = direction;
            }
            pub fn get_fuel(&self) -> i32 {
                self.fuel.current_fuel
            }
            pub fn set_fuel(&mut self, arg: i32) {
                self.fuel.current_fuel = arg;
            }
        }
        impl std::fmt::Display for SpaceShip<i32> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Spaceship coordinates: [x:{},y:{}], current direction: {}, current velocity: [x:{},y:{}], current a.velocity: {}\n
                    current fuel:{},current consumption:{}",
                    self.coord.x, self.coord.y, self.direction , self.velocity.x,
                    self.velocity.y , self.angular_velocity,self.fuel.current_fuel, self.fuel.consumpt  )
            }
        }
        // implement base interface
        impl Movable for SpaceShip<i32> {
            type Coordinates = Point2D<i32>;
            fn try_get_position(&self) -> Result<Self::Coordinates, ErrorProcessing> {
                Ok(Self::Coordinates {
                    // nofail
                    x: self.coord.x,
                    y: self.coord.y,
                })
            }
            fn try_get_velocity(&self) -> Result<Self::Coordinates, ErrorProcessing> {
                Ok(self.velocity)
            }
            fn try_set_position(
                &mut self,
                vector: Self::Coordinates,
            ) -> Result<(), ErrorProcessing> {
                self.coord.x = vector.x;
                self.coord.y = vector.y;
                Ok(())
            }
        }
        impl Rotable for SpaceShip<i32> {
            fn get_directions_number(&self) -> i8 {
                self.direction_numbers.abs()
            }
            fn get_angular_velocity(&self) -> i8 {
                self.angular_velocity
            }
            fn try_get_direction(&self) -> Result<i8, ErrorProcessing> {
                Ok(self.direction) // nofail
            }

            fn try_set_direction(&mut self, direct: i8) -> Result<(), ErrorProcessing> {
                self.direction = direct;
                Ok(())
            }
        }

        //------------------------------------------------------
        //------------------------------------------------------
        //------------------------------------------------------
        impl RotateObj for SpaceShip<i32> {} // default

        impl MoveObj for SpaceShip<i32> {} // default
        impl Consumable for SpaceShip<i32> {
            fn check(&self) -> Result<(), ErrorProcessing> {
                if self.fuel.current_fuel - self.fuel.consumpt <= 0 {
                    Err(ErrorProcessing::ErrCheck)
                } else {
                    Ok(())
                }
            }

            fn consume(&mut self) {
                self.fuel.current_fuel -= self.fuel.consumpt;
            }
        }

        //                     slog::info!(logger , "PRINTER2:error happened!"; "error:" => err.to_string());
        impl GeneralPurpose for SpaceShip<i32> {
            fn enable_logging(&mut self) {
                if !self.logger_enabled {
                    self.logger_enabled = true;
                    slog::info!(*self.logger, "logging enabled!");
                }
            }
        }
    }
    //------------------------------------------------------
    //------------------------------------------------------
    //------------------------------------------------------
    // tests
    #[cfg(test)]
    pub mod testing_implement {
        use super::test_moving_object::*;
        use super::*;
        use lib_game::actions::commanding::*;
        use mockall::predicate::*;
        use mockall::*;
        use rstest::*;
        use slog::Drain;
        use std::convert::{From, Into};
        use std::sync::Mutex;

        #[fixture]
        pub fn get_spaceship_instance() -> SpaceShip<i32> {
            let mut sh1 = SpaceShip::new();
            let init_post = Point2D { x: 0, y: 0 };
            let init_vel = Point2D { x: 1, y: 1 };
            sh1.set_initial(init_post, init_vel);
            assert_eq!(
                Into::<(i32, i32)>::into(sh1.try_get_position().unwrap()),
                init_post.into()
            );
            assert_eq!(
                Into::<(i32, i32)>::into(sh1.try_get_velocity().unwrap()),
                init_vel.into()
            );
            sh1.set_initial_angular(0);
            sh1.set_initial_direction(0);
            sh1
        }

        #[rstest]
        fn test_command_logging_no_errors() {
            let mut sh1 = get_spaceship_instance();
            let mut shema1 = Schema::default();
            shema1.assign_logger(std::rc::Rc::clone(&sh1.logger));
            shema1.push(&CommandStore::WriteLog);
            shema1.push(&CommandStore::MoveCommand);
            shema1.execute(&mut sh1).unwrap();
            println!("finish no errors");
        }
        /* я не знаю как вменяемо реализовать тест,(ибо тут проброска ошибок по умолчанию хорошо
         * работает), кроме как введения кастомной ошибки
         * иначе получается жеское натягивание совы на глобус */
        #[rstest]
        fn test_command_logging_errors() {
            let mut sh1 = get_spaceship_instance();
            let mut shema1 = Schema::default();
            shema1.assign_logger(std::rc::Rc::clone(&sh1.logger)); //ok
            shema1.push(&CommandStore::WriteLog);
            shema1.push(&CommandStore::SomeErrorPerform); // pass some that returns error
                                                          //            assert_eq!(shema1.execute(&mut sh1).unwrap_err(), ErrorProcessing::Err);
        }
        // OH MY LORD!
        //#################################################################################################
        //#################################################################################################
        //#################################################################################################
        //         impl<T> Commandable for T where T: MoveObj + RotateObj + Consumable + GeneralPurpose + ?Sized {}
        mock! {
        pub TestErrors{}
        impl MoveObj for TestErrors{
            fn execute_move(&mut self) -> Result<(), ErrorProcessing>;
        }
        impl RotateObj for TestErrors{
            fn execute_rotate(&mut self) -> Result<(), ErrorProcessing>;
        }
        impl Consumable for TestErrors{
            fn check(&self) -> Result<(), ErrorProcessing>;
            fn consume(&mut self);

        }
        impl GeneralPurpose for TestErrors{
              fn enable_logging(&mut self);
        }
        impl ErrorHandler for TestErrors{
                fn process_error(&self, err: ErrorProcessing);
        }

        impl Movable for TestErrors{
            type Coordinates = Point2D<i32>;
            fn try_get_position(&self) -> Result<<Self as Movable>::Coordinates, ErrorProcessing>;
            fn try_get_velocity(&self) -> Result<<Self as Movable>::Coordinates, ErrorProcessing>;
            fn try_set_position(&mut self, vector: <Self as Movable>::Coordinates) -> Result<(), ErrorProcessing>;
        }
        impl Rotable for TestErrors{
            fn get_directions_number(&self) -> i8;
            fn get_angular_velocity(&self) -> i8;
            fn try_get_direction(&self) -> Result<i8, ErrorProcessing>;
            fn try_set_direction(&mut self, direct: i8) -> Result<(), ErrorProcessing>;
            }
        impl Command for TestErrors{
            fn perform(
                &self,
                obj: &mut dyn Commandable<Coordinates = Point2D<i32>>,
            ) -> Result<(), ErrorProcessing>;
            }
        }

        //#################################################################################################
        //#################################################################################################
        //#################################################################################################
        /*     #[test]
                fn test_command_logging_mock_log_enable() {
                    let mut shema1 = Schema::default();
                    let mut mocked_obj = MockTestErrors::new();
                    mocked_obj.expect_enable_logging().times(1).returning(|| ());
                    shema1.push(&CommandStore::WriteLog);
                    shema1.execute(&mut mocked_obj).unwrap()
                }
                #[test]
                fn test_command_logging_some_error() {
                    let decorator = slog_term::TermDecorator::new().build();
                    let drain = Mutex::new(slog_term::FullFormat::new(decorator).build()).fuse();
                    let log = std::rc::Rc::new(slog::Logger::root(drain, slog::o!()));

                    let mut shema1 = Schema::default();
                    shema1.assign_logger(log);
                    let mut mocked_obj = MockTestErrors::new();
                    let _some_coord: Point2D<i32> = Point2D::default();
                    mocked_obj.expect_enable_logging().times(1).returning(|| ());
                    mocked_obj
                        .expect_execute_move()
                        .times(1)
                        .returning(|| Err(ErrorProcessing::ErrGetInfo));
                    //             mocked_obj.expect_perform().times(2);
                    //             mocked_obj.expect_process_error().times(1);
                    shema1.push(&CommandStore::WriteLog);
                    shema1.push(&CommandStore::MoveCommand);
                    shema1.execute(&mut mocked_obj).unwrap()
                }
        */
        #[rstest]
        fn test_rotate_and_move_commands() {
            let mut sh1 = get_spaceship_instance();
            sh1.set_initial_direction(3);
            sh1.set_initial_angular(2);
            sh1.set_initial((5, 5).into(), (-2, -2).into());
            let mut shema1 = Schema::new();
            shema1.push(&CommandStore::WriteLog);
            shema1.push(&CommandStore::MoveCommand);
            shema1.push(&CommandStore::RotateCommand);
            shema1.execute(&mut sh1).unwrap();
            assert_eq!(
                Into::<(i32, i32)>::into(sh1.try_get_position().unwrap()),
                (3, 3)
            );
            assert_eq!(
                sh1.try_get_direction().unwrap(),
                (3 + 2) % DIRECTION_NUMBERS
            );
            assert_eq!(sh1.try_get_direction().unwrap(), 5);
        }
        /*           mocked_obj
                       .expect_try_get_direction()
                       .times(1)
                       .returning(|| Ok(0));
                   mocked_obj
                       .expect_try_get_velocity()
                       .times(1)
                       .returning(move || Ok(some_point));
                   mocked_obj
                       .expect_try_get_position()
                       .times(1)
                       .returning(move || Ok(some_point));
                   mocked_obj
                       .expect_try_set_position()
                       .times(1)
                       .returning(move |_some_coord| Err(ErrorProcessing::ErrSetting));
        */

        /*
        #[rstest]
        fn test_fuel_macrocommand() {
            let mut sh1 = get_spaceship_instance();
            sh1.set_initial_direction(3);
            sh1.set_initial_angular(2);
            sh1.set_initial((5, 5).into(), (-2, -2).into());
            sh1.set_fuel(5);
            let shema1 = Schema::move_and_consume_fuel().unwrap(); //
            assert!(shema1.execute(&mut sh1).is_ok());
            assert_eq!(sh1.get_fuel(), 4); // check fuel consumption
            assert_eq!(
                // check potision change
                Into::<(i32, i32)>::into(sh1.try_get_position().unwrap()),
                (3, 3)
            );
        }*/
        /*
        #[rstest]
        fn test_fuel_macrocommand_error() {
            let mut sh1 = get_spaceship_instance();
            sh1.set_initial_direction(3);
            sh1.set_initial_angular(2);
            sh1.set_initial((5, 5).into(), (-2, -2).into());
            sh1.set_fuel(0);
            let shema1 = Schema::move_and_consume_fuel().unwrap(); //
            assert_eq!(
                // check we got error
                shema1.execute(&mut sh1).unwrap_err(),
                ErrorProcessing::ErrCheck
            );
            assert_eq!(
                Into::<(i32, i32)>::into(sh1.try_get_position().unwrap()),
                (5, 5)
            );
        }*/

        //#[rstest]
        //fn test_log() {
        /*            let decorator = slog_term::TermDecorator::new().build();
        let drain = Mutex::new(slog_term::FullFormat::new(decorator).build()).fuse();
        let log = slog::Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")));
        trace!(log, "logging a trace message");
        debug!(log, "debug values"; "x" => 1, "y" => -1);
        info!(log, "some interesting info"; "where" => "right here");
        warn!(log, "be cautious!"; "why" => "you never know...");
        error!(log, "wrong {}", "foobar"; "type" => "unknown");
        crit!(log, "abandoning test"); */
    }
}

/*
        mock! {
        pub TestErrors{}
        impl MoveObj for TestErrors{
        }
        impl RotateObj for TestErrors{
            fn execute_rotate(&mut self) -> Result<(), ErrorProcessing>;
        }
        impl Consumable for TestErrors{
            fn check(&self) -> Result<(), ErrorProcessing>;
            fn consume(&mut self);

        }
        impl GeneralPurpose for TestErrors{
            fn enable_logging(&mut self);
        }
        impl Movable for TestErrors{
            type Coordinates = Point2D<i32>;
            fn try_get_position(&self) -> Result<<Self as Movable>::Coordinates, ErrorProcessing>;
            fn try_get_velocity(&self) -> Result<<Self as Movable>::Coordinates, ErrorProcessing>;
            fn try_set_position(&mut self, vector: <Self as Movable>::Coordinates) -> Result<(), ErrorProcessing>;
        }
        impl Rotable for TestErrors{
            fn get_directions_number(&self) -> i8;
            fn get_angular_velocity(&self) -> i8;
            fn try_get_direction(&self) -> Result<i8, ErrorProcessing>;
            fn try_set_direction(&mut self, direct: i8) -> Result<(), ErrorProcessing>;
            }
        impl Command for TestErrors{
            fn perform(
                &self,
                obj: &mut dyn Commandable<Coordinates = Point2D<i32>>,
            ) -> Result<(), ErrorProcessing>;
            }
        }
*/
