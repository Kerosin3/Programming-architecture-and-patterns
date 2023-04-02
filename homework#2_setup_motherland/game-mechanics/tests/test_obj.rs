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
            fn get_position(&self) -> Self::Coordinates {
                self.coord
            }
            fn get_velocity(&self) -> Self::Coordinates {
                self.velocity
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
        impl MoveObj for SpaceShip<i32> {} // default
    }
    #[rstest]
    #[case((12_i32,5_i32),(-7_i32,3_i32),(5_i32,8_i32))]
    fn it_works(
        #[case] input_pos: (i32, i32),
        #[case] input_vel: (i32, i32),
        #[case] expected_pos: (i32, i32),
    ) {
        use test_moving_object::*;
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
            Into::<(i32, i32)>::into(sh1.get_position()),
            init_post.into()
        );
        assert_eq!(
            Into::<(i32, i32)>::into(sh1.get_velocity()),
            init_vel.into()
        );
        assert!(sh1.execute().is_ok());
        assert_eq!(
            Into::<(i32, i32)>::into(sh1.get_position()),
            expected_pos.into()
        );
    }
}
