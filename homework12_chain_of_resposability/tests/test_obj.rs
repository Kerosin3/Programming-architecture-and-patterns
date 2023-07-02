use libcommand::commanding::*;
use std::rc::Rc;

#[cfg(test)]
#[derive(Default)]
struct SpaceShip {
    coord: i32,
    vel: i32,
}

impl MainObject for SpaceShip {
    fn init() -> Self {
        Self { coord: 0, vel: 2 }
    }

    fn setup(&mut self) {
        todo!()
    }
}

#[test]
fn testme() {
    let spaceship = SpaceShip::init();
}
