fn main() {
    let circ = ShapeFactory::new_shape(&ShapeTypes::Circle);
    let rec = ShapeFactory::new_shape(&ShapeTypes::Rectangle);
    circ.draw();
    rec.draw();
}

trait Shape {
    fn draw(&self);
}

enum ShapeTypes {
    Rectangle,
    Circle,
}

struct Rectange {}

impl Shape for Rectange {
    fn draw(&self) {
        println!("drawing rectangle!");
    }
}

struct Circle {}

impl Shape for Circle {
    fn draw(&self) {
        println!("drawing circle");
    }
}
struct ShapeFactory;
impl ShapeFactory {
    fn new_shape(s: &ShapeTypes) -> Box<dyn Shape> {
        match s {
            ShapeTypes::Rectangle => Box::new(Rectange {}),
            ShapeTypes::Circle => Box::new(Circle {}),
        }
    }
}
