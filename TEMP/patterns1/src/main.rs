fn main() {
    let struct0 = GenericStruct { fierd: Class1() };
    struct0.func1();
    let struct1 = GenericStruct { fierd: Class2() };
    struct1.funcSPECIFIC();
}

struct GenericStruct<P: Trait1> {
    fierd: P,
}

impl<P: Trait1> GenericStruct<P> {
    fn func1(&self) {
        self.fierd.execute();
    }
}

impl GenericStruct<Class2> {
    fn funcSPECIFIC(&self) {
        println!("specific function!");
    }
}

trait Trait1 {
    fn execute(&self);
}

struct Class1();
struct Class2();

impl Trait1 for Class1 {
    fn execute(&self) {
        println!("class 1");
    }
}
impl Trait1 for Class2 {
    fn execute(&self) {
        println!("class 2");
    }
}
