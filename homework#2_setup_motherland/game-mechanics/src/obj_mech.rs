pub mod object_moving {
    use mockall::predicate::*;
    use mockall::*;
    use std::fmt::Display;
    use thiserror::Error;
    pub trait Movable {
        type Coordinates;
        fn try_get_position(&self) -> Result<Self::Coordinates, ErrorMovindObject>;
        fn try_get_velocity(&self) -> Result<Self::Coordinates, ErrorMovindObject>;
        fn try_set_position(&mut self, vector: Self::Coordinates) -> Result<(), ErrorMovindObject>;
    }
    pub trait MoveObj: Movable {
        fn execute_move(&mut self) -> Result<(), ErrorMovindObject>
        where
            <Self as Movable>::Coordinates: std::ops::Add<Output = <Self as Movable>::Coordinates>,
        {
            let svec = std::ops::Add::add(self.try_get_position()?, self.try_get_velocity()?);
            self.try_set_position(svec)?;
            Ok(())
        }
    }
    pub trait Rotable {
        type Coordinates;
        fn get_direction_number(&self);
        fn get_angular_velocity(&self);
        fn try_get_direction(&self) -> Result<Directions, ErrorMovindObject>;
        fn try_set_direction(&self) -> Result<(), ErrorMovindObject>;
    }
    pub trait RotateObj: Rotable {
        fn execute_rotate(&mut self) -> Result<(), ErrorMovindObject> {
            todo!()
        }
    }
    #[derive(Copy, Clone, Debug)]
    pub enum Directions {
        _0,
        _45,
        _90,
        _135,
        _180,
        _225,
        _270,
        _315,
        _360,
    }

    impl std::ops::Add for Directions {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            todo!()
        }
    }
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
    pub enum ErrorMovindObject {
        #[error("generic error")]
        Err,
        #[error("error getting info")]
        ErrGetInfo,
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
    mock! {
        TestMe {}
        impl Trait1 for TestMe{
            fn func1(&self) -> Result<(), SomeError>;
            fn func2(&self) -> Result<(), SomeError>;

        }
        impl Trait2 for TestMe{
            fn funcX(&self) -> Result<(), SomeError>;
        }
    }
}
