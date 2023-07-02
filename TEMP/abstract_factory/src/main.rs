fn main() {
    let winfactory = WinFactory;
    let buttonw = winfactory.create_button();
    let checkboxnw = winfactory.create_checkbox();
}
trait GUIFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}

trait Button {
    fn paint(&self);
}

trait Checkbox {
    fn check(&self);
}

struct WinFactory;
impl GUIFactory for WinFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WinButton {})
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WinCheckbox {})
    }
}

struct WinButton;
impl Button for WinButton {
    fn paint(&self) {
        println!("painting win button");
    }
}
struct WinCheckbox;
impl Checkbox for WinCheckbox {
    fn check(&self) {
        println!("checking Win");
    }
}
