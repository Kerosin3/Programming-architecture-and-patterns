#[cfg(test)]
#[allow(unused_imports)]
mod oject_interface_test {

    //    use lib_game_mechanics::obj_mech::ObjectMoving::*;
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
        #[derive(Copy, Clone, Debug, Default)]
        pub struct SpaceShip<T> {
            coord: Point2D<T>,
            velocity: Point2D<T>,
        }
        impl SpaceShip<i32> {
            pub fn new() -> Self {
                let dp = Point2D::default();
                Self {
                    coord: dp,
                    velocity: dp,
                }
            }
            pub fn set_initial(&mut self, pos: Point2D<i32>, vel: Point2D<i32>) {
                self.coord = pos;
                self.velocity = vel;
            }
        }
        // implement base interface
        impl Movable for SpaceShip<i32> {
            type Coordinates = Point2D<i32>;
            fn try_get_position(&self) -> Result<Self::Coordinates, ErrorMovindObject> {
                let _magic_value = Point2D { x: 1, y: -1 };
                match self.coord {
                    p1 if ((p1.x != 1_i32) & (p1.y != -1_i32)) => Ok(self.coord),
                    _ => Err(ErrorMovindObject::ErrGetInfo),
                }
            }
            fn try_get_velocity(&self) -> Result<Self::Coordinates, ErrorMovindObject> {
                Ok(self.velocity)
            }
            fn try_set_position(
                &mut self,
                vector: Self::Coordinates,
            ) -> Result<(), ErrorMovindObject> {
                self.coord.x = vector.x;
                self.coord.y = vector.y;
                Ok(())
            }
        }
        mock! {
            Something1 {}
            impl Movable for Something1{
                type Coordinates = Point2D<i32>;
                fn try_get_position(&self) -> Result<<Self as Movable>::Coordinates, ErrorMovindObject>;
                fn try_get_velocity(&self) -> Result<<Self as Movable>::Coordinates, ErrorMovindObject>;
                fn try_set_position(&mut self, vector: <Self as Movable>::Coordinates) -> Result<(), ErrorMovindObject>;
                }
            impl MoveObj for Something1 {
                    fn execute_move(&mut self) -> Result<(), ErrorMovindObject>{
                        try_get_position();
                        try_get_velocity();
                }
            }
        }
        impl MoveObj for SpaceShip<i32> {} // default
                                           //----------------------------------
                                           //----------------------------------
        #[test]
        fn test_error_getting_position() {
            use mockall::predicate::*;
            use mockall::*;

            let expectd = Err(ErrorMovindObject::Err);

            let mut mock_moving = Box::new(MockSomething1::new());
            mock_moving
                .expect_execute_move()
                .times(1)
                .returning(move || expectd);
            mock_moving.execute_move().unwrap();
        }
    }
    mod testing_implement {
        use super::test_moving_object::*;
        use super::*;
        use mockall::predicate::*;
        use mockall::*;
        use rstest::rstest;
        use std::convert::{From, Into};
        /*
        #[rstest]
        #[case((12_i32,5_i32),(-7_i32,3_i32),(5_i32,8_i32))]
        fn it_works(
            #[case] input_pos: (i32, i32),
            #[case] input_vel: (i32, i32),
            #[case] expected_pos: (i32, i32),
        ) {
            let mut sh1 = SpaceShip::new();
            let init_post = Point2D {
                x: input_pos.0,
                y: input_pos.1,
            };
            let init_vel = Point2D {
                x: input_vel.0,
                y: input_vel.1,
            };
            sh1.set_initial(init_post, init_vel);
            assert_eq!(
                Into::<(i32, i32)>::into(sh1.try_get_position().unwrap()),
                init_post.into()
            );
            assert_eq!(
                Into::<(i32, i32)>::into(sh1.try_get_velocity().unwrap()),
                init_vel.into()
            );
            assert!(sh1.execute_move().is_ok());
            assert_eq!(
                Into::<(i32, i32)>::into(sh1.try_get_position().unwrap()),
                expected_pos.into()
            );
        }*/
    }
}
