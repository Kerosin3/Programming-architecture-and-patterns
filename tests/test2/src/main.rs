use std::rc::Rc;

struct somestruct{
    x: Rc<Box<String>>
}

fn main() {
    let a = somestruct;
}
