pub mod object_moving {
    pub trait Movable {
        type Coordinates;
        fn get_position(&self) -> Self::Coordinates;
        fn get_velocity(&self) -> Self::Coordinates;
        fn try_set_position(&mut self, vector: Self::Coordinates) -> Result<(), ErrorMovindObject>;
    }
    pub trait MoveObj: Movable {
        fn execute(&mut self) -> Result<(), ErrorMovindObject>
        where
            <Self as Movable>::Coordinates: std::ops::Add<Output = Self::Coordinates>,
        {
            let svec = std::ops::Add::add(self.get_position(), self.get_velocity());
            self.try_set_position(svec)?;
            Ok(())
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
    #[derive(Copy, Clone, Debug)]
    pub enum ErrorMovindObject {
        Err,
    }
}
