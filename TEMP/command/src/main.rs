use std::collections::HashMap;
use std::rc::Rc;
fn main() {
    let tv = TV::new();
    let mut remote_control = TVController::new();
    remote_control.press_button(0);
    remote_control.set_command(1, Box::new(TVOnCommand::new(Rc::clone(&tv))));
    remote_control.set_command(2, Box::new(TVOffComand::new(Rc::clone(&tv))));
    remote_control.press_button(1);
    remote_control.press_button(2);
}

trait Command {
    fn execute(&self);
}

struct TV;
impl TV {
    fn new() -> Rc<Self> {
        Rc::new(TV)
    }
    fn on(&self) {
        println!("TV IS ON!");
    }
    fn off(&self) {
        println!("TV IS Off!");
    }
}
//-------------------------------------
// On COmmand
struct TVOnCommand {
    tv: Rc<TV>,
}
impl TVOnCommand {
    fn new(tv: Rc<TV>) -> Self {
        Self { tv }
    }
}
impl Command for TVOnCommand {
    fn execute(&self) {
        self.tv.on()
    }
}
//-------------------------------------
struct TVOffComand {
    tv: Rc<TV>,
}
impl TVOffComand {
    fn new(tv: Rc<TV>) -> Self {
        Self { tv }
    }
}

impl Command for TVOffComand {
    fn execute(&self) {
        self.tv.off()
    }
}

struct TVController {
    Commands: HashMap<i32, Box<dyn Command>>,
}
impl TVController {
    fn new() -> Self {
        Self {
            Commands: HashMap::default(),
        }
    }
    fn set_command(&mut self, idx: i32, cmd: Box<dyn Command>) {
        self.Commands.insert(idx, cmd);
    }
    fn press_button(&self, idx: i32) {
        if let Some(cmd) = self.Commands.get(&idx) {
            cmd.execute();
        } else {
            println!("skipping");
        }
    }
}
