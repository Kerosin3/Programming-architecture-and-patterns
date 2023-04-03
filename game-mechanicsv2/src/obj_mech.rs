pub mod object_moving {
    use mockall::predicate::*;
    use mockall::*;
    //     use std::fmt::Display;
    use thiserror::Error;
    //--------------------------------------------------------------------------------------
    //--------------------------------------------------------------------------------------
    //--------------------------------------------------------------------------------------
    //--------------------------------------------------------------------------------------
    pub trait Commandable: MoveObj + RotateObj {}
    pub trait Movable {
        type Coordinates;
        fn try_get_position(&self) -> Result<Self::Coordinates, ErrorProcessing>;
        fn try_get_velocity(&self) -> Result<Self::Coordinates, ErrorProcessing>;
        fn try_set_position(&mut self, vector: Self::Coordinates) -> Result<(), ErrorProcessing>;
    }
    pub trait MoveObj: Movable {
        fn execute_move(&mut self) -> Result<(), ErrorProcessing>
        where
            <Self as Movable>::Coordinates: std::ops::Add<Output = <Self as Movable>::Coordinates>,
        {
            let svec = std::ops::Add::add(self.try_get_position()?, self.try_get_velocity()?);
            self.try_set_position(svec)?;
            Ok(())
        }
    }
    //--------------------------------------------------------------------------------------
    pub trait Rotable {
        fn get_directions_number(&self) -> i8;
        fn get_angular_velocity(&self) -> i8;
        fn try_get_direction(&self) -> Result<i8, ErrorProcessing>;
        fn try_set_direction(&mut self, direct: i8) -> Result<(), ErrorProcessing>;
    }
    pub trait RotateObj: Rotable {
        fn execute_rotate(&mut self) -> Result<(), ErrorProcessing> {
            let d_number = 8_i8;
            let t1 = self.try_get_direction()? + self.get_angular_velocity();
            let t2 = t1 % (i8::from(d_number));
            self.try_set_direction(t2)?;
            Ok(())
        }
    }
    //--------------------------------------------------------------------------------------
    //--------------------------------------------------------------------------------------
    //--------------------------------------------------------------------------------------
    //--------------------------------------------------------------------------------------
    #[derive(Copy, Clone, Debug, Default)]
    pub struct Point2D<T> {
        pub x: T,
        pub y: T,
    }
    impl std::convert::From<Point2D<i32>> for (i32, i32) {
        fn from(value: Point2D<i32>) -> Self {
            (value.x, value.y)
        }
    }
    impl std::convert::From<(i32, i32)> for Point2D<i32> {
        fn from(value: (i32, i32)) -> Self {
            Self {
                x: value.0,
                y: value.1,
            }
        }
    }

    impl std::ops::Add for Point2D<i32> {
        type Output = Point2D<i32>;
        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    #[derive(Copy, Clone, Debug, Error, PartialEq)]
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
    }
    pub trait Trait1 {
        fn func1(&self) -> Result<(), SomeError>;
        fn func2(&self) -> Result<(), SomeError>;
    }

    pub trait Trait2: Trait1 {
        fn funcX(&self) -> Result<(), SomeError> {
            self.func1()?;
            self.func2()?;
            Ok(())
        }
    }
    pub enum SomeError {
        Err,
    }
}