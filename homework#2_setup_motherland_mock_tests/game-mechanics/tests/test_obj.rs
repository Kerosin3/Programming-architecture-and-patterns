#[cfg(test)]
#[allow(unused_imports)]
mod oject_interface_test {

    use lib_game_mech::obj_mech::object_moving::*;
    use mockall::predicate::*;
    use mockall::*;
    use rstest::rstest;
    #[cfg(test)]
    mod test_moving_object {
        use super::*;
        use mockall::predicate::*;
        use mockall::*;
        use rstest::rstest;
        use std::convert::{From, Into};
        //------------------------------------------------------
        //------------------------------------------------------
        //example class
        #[derive(Copy, Clone, Debug, Default)]
        pub struct SpaceShip<T> {
            coord: Point2D<T>,
            velocity: Point2D<T>,
            direction_numbers: i8,
            angular_velocity: i8,
            direction: i8,
        }
        impl SpaceShip<i32> {
            pub fn new() -> Self {
                let dp = Point2D::default();
                Self {
                    coord: dp,
                    velocity: dp,
                    direction_numbers: DIRECTION_NUMBERS,
                    angular_velocity: 0,
                    direction: 2,
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
        }
        impl std::fmt::Display for SpaceShip<i32> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Spaceship coordinates: [x:{},y:{}], current direction: {}, current velocity: [x:{},y:{}], current a.velocity: {}",
                    self.coord.x, self.coord.y, self.direction , self.velocity.x, self.velocity.y , self.angular_velocity  )
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
    }
    //------------------------------------------------------
    //------------------------------------------------------
    //------------------------------------------------------
    // tests
    #[cfg(test)]
    pub mod testing_implement {
        use super::test_moving_object::*;
        use super::*;
        use mockall::predicate::*;
        use mockall::*;
        use rstest::*;
        use std::convert::{From, Into};

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
        // test position change after executing move
        #[rstest]
        #[case((12_i32,5_i32),(-7_i32,3_i32),(5_i32,8_i32))]
        fn test_moving(
            #[case] input_pos: (i32, i32),
            #[case] input_vel: (i32, i32),
            #[case] expected_pos: (i32, i32),
        ) {
            let mut sh1 = get_spaceship_instance();
            sh1.set_initial(input_pos.into(), input_vel.into());
            assert!(sh1.execute_move().is_ok());
            assert_eq!(
                Into::<(i32, i32)>::into(sh1.try_get_position().unwrap()),
                expected_pos.into()
            );
        }
        // test direction change after execute rotate
        #[rstest]
        #[case(7_i8, 3_i8)]
        fn test_rotating(#[case] input_angl_vel: i8, #[case] input_direction: i8) {
            let mut sh1 = get_spaceship_instance();
            sh1.set_initial_direction(input_direction);
            sh1.set_initial_angular(input_angl_vel);
            sh1.execute_rotate().unwrap();
            assert_eq!(
                sh1.try_get_direction().unwrap(),
                (input_direction + input_angl_vel) % DIRECTION_NUMBERS
            );
        }
        // test direction change after execute rotate
        #[rstest]
        #[case(0_i8, 3_i8)]
        fn test_rotating_no_velocity(#[case] input_angl_vel: i8, #[case] input_direction: i8) {
            let mut sh1 = get_spaceship_instance();
            sh1.set_initial_direction(input_direction);
            sh1.set_initial_angular(input_angl_vel);
            sh1.execute_rotate().unwrap();
            assert_eq!(sh1.try_get_direction().unwrap(), input_direction);
            assert_eq!(sh1.get_angular_velocity(), input_angl_vel);
        }
        // test neg rotation
        #[rstest]
        #[case(-2_i8, 5_i8)]
        fn test_rotating_neg_velocity(#[case] input_angl_vel: i8, #[case] input_direction: i8) {
            let mut sh1 = get_spaceship_instance();
            sh1.set_initial_direction(input_direction);
            sh1.set_initial_angular(input_angl_vel);
            sh1.execute_rotate().unwrap();
            assert_eq!(
                sh1.try_get_direction().unwrap(),
                (input_direction + input_angl_vel) % DIRECTION_NUMBERS
            );
            assert_eq!(sh1.get_angular_velocity(), input_angl_vel);
        }
        // test rotation and moving
        #[rstest]
        fn test_rotate_and_move() {
            let mut sh1 = get_spaceship_instance();
            sh1.set_initial_direction(3);
            sh1.set_initial_angular(2);
            sh1.set_initial((5, 5).into(), (-2, -2).into());
            sh1.execute_move().unwrap();
            sh1.execute_rotate().unwrap();
            assert_eq!(
                Into::<(i32, i32)>::into(sh1.try_get_position().unwrap()),
                (3, 3).into()
            );
            assert_eq!(
                sh1.try_get_direction().unwrap(),
                (3 + 2) % DIRECTION_NUMBERS
            );
            assert_eq!(sh1.try_get_direction().unwrap(), 5);
        }
        // test error cant get positions
        // ---------------------MOCK TESTS-------------------------------//
        // --------------------------------------------------------------//
        // --------------------------------------------------------------//
        // --------------------------------------------------------------//

        mock! {
        CantGetPosition {}
        impl Movable for CantGetPosition{
            type Coordinates = Point2D<i32>;
            fn try_get_position(&self) -> Result<<Self as Movable>::Coordinates, ErrorProcessing>;
            fn try_get_velocity(&self) -> Result<<Self as Movable>::Coordinates, ErrorProcessing>;
            fn try_set_position(&mut self, vector: <Self as Movable>::Coordinates) -> Result<(), ErrorProcessing>;
            }
        impl MoveObj for CantGetPosition {}
        }

        #[test]
        fn test_mocking_cant_move() {
            let some_point = Point2D::default();
            let mut mocked_obj = Box::new(MockCantGetPosition::new());
            mocked_obj
                .expect_try_get_position()
                .times(1)
                .returning(|| Err(ErrorProcessing::ErrGetInfo));
            mocked_obj
                .expect_try_get_velocity()
                .times(0)
                .returning(move || Ok(some_point));
            assert_eq!(
                mocked_obj.execute_move().unwrap_err(),
                ErrorProcessing::ErrGetInfo // forwarded
            )
        }
        // --------------------------------------------------------------//
        // --------------------------------------------------------------//
        // --------------------------------------------------------------//

        mock! {
        CantGetVelocity {}
        impl Movable for CantGetVelocity{
            type Coordinates = Point2D<i32>;
            fn try_get_velocity(&self) -> Result<<Self as Movable>::Coordinates, ErrorProcessing>;
            fn try_get_position(&self) -> Result<<Self as Movable>::Coordinates, ErrorProcessing>;
            fn try_set_position(&mut self, vector: <Self as Movable>::Coordinates) -> Result<(), ErrorProcessing>;
            }
        impl MoveObj for CantGetVelocity {}
        }

        #[test]
        fn test_mocking_cant_get_velocity() {
            let some_point = Point2D::default();
            let mut mocked_obj = Box::new(MockCantGetVelocity::new());
            mocked_obj
                .expect_try_get_velocity()
                .times(1)
                .returning(|| Err(ErrorProcessing::ErrGetInfo));
            mocked_obj
                .expect_try_get_position()
                .times(1)
                .returning(move || Ok(some_point));

            assert_eq!(
                mocked_obj.execute_move().unwrap_err(),
                ErrorProcessing::ErrGetInfo
            )
        }
        // --------------------------------------------------------------//
        // --------------------------------------------------------------//
        // --------------------------------------------------------------//
        mock! {
        CantChangePosition {}
        impl Movable for CantChangePosition{
            type Coordinates = Point2D<i32>;
            fn try_get_velocity(&self) -> Result<<Self as Movable>::Coordinates, ErrorProcessing>;
            fn try_get_position(&self) -> Result<<Self as Movable>::Coordinates, ErrorProcessing>;
            fn try_set_position(&mut self, vector: <Self as Movable>::Coordinates) -> Result<(), ErrorProcessing>;
            }
        impl MoveObj for CantChangePosition {}
        }

        #[test]
        fn test_mocking_cant_change_position() {
            let _some_coord: Point2D<i32> = Point2D::default();
            let some_point = Point2D::default();
            let mut mocked_obj = Box::new(MockCantChangePosition::new());
            mocked_obj
                .expect_try_get_velocity()
                .times(1)
                .returning(move || Ok(some_point.clone()));
            mocked_obj
                .expect_try_get_position()
                .times(1)
                .returning(move || Ok(some_point));

            mocked_obj
                .expect_try_set_position()
                .times(1)
                .returning(move |_some_coord| Err(ErrorProcessing::ErrSetting));
            assert_eq!(
                mocked_obj.execute_move().unwrap_err(),
                ErrorProcessing::ErrSetting
            )
        }
        // --------------------------------------------------------------//
        // --------------------------------------------------------------//
        // --------------------------------------------------------------//

        mock! {
        CantGetDirection {}
        impl Rotable for CantGetDirection{
            fn get_directions_number(&self) -> i8;
            fn get_angular_velocity(&self) -> i8;
            fn try_get_direction(&self) -> Result<i8, ErrorProcessing>;
            fn try_set_direction(&mut self, direct: i8) -> Result<(), ErrorProcessing>;
            }
        impl RotateObj for CantGetDirection {}
        }

        #[test]
        fn test_mock_cant_get_direction() {
            let mut mocked_obj = Box::new(MockCantGetDirection::new());
            mocked_obj
                .expect_try_get_direction()
                .times(1)
                .returning(|| Err(ErrorProcessing::ErrGetInfo));
            mocked_obj
                .expect_get_angular_velocity()
                .times(0) // paniced earlier
                .returning(|| 1_i8);
            mocked_obj
                .expect_try_set_direction()
                .times(0)
                .returning(move |_| Ok(()));
            assert_eq!(
                mocked_obj.execute_rotate().unwrap_err(),
                ErrorProcessing::ErrGetInfo
            )
        }
        // --------------------------------------------------------------//
        // --------------------------------------------------------------//
        // --------------------------------------------------------------//
        mock! {
        CantSetDirection {}
        impl Rotable for CantSetDirection{
            fn get_directions_number(&self) -> i8;
            fn get_angular_velocity(&self) -> i8;
            fn try_get_direction(&self) -> Result<i8, ErrorProcessing>;
            fn try_set_direction(&mut self, direct: i8) -> Result<(), ErrorProcessing>;
            }
        impl RotateObj for CantSetDirection {}
        }

        #[test]
        fn test_mock_cant_set_direction() {
            let mut mocked_obj = Box::new(MockCantSetDirection::new());
            mocked_obj
                .expect_try_get_direction()
                .times(1)
                .returning(|| Ok(3_i8));
            mocked_obj
                .expect_get_angular_velocity()
                .times(1) //
                .returning(|| 1_i8);
            mocked_obj
                .expect_try_set_direction()
                .times(1)
                .returning(move |_| Err(ErrorProcessing::ErrSetting));
            assert_eq!(
                mocked_obj.execute_rotate().unwrap_err(),
                ErrorProcessing::ErrSetting
            )
        }
        // --------------------------------------------------------------//
        // --------------------------------------------------------------//
        // --------------------------------------------------------------//
    }
}
