mod modules;
use modules::module1::enum_example::*;
use modules::module1::example1::Hehe;
use modules::module1::example2::TupleStruct;
use modules::module1::example_composition::*;
fn main() {
    let mut a = Hehe::new(42);
    a.increase_all(555);
    println!("fresh instance a: {:#?}", a);
    //--------------------------------//
    let mut b = TupleStruct::new();
    println!("fresh instance b: {:#?}", b);
    //-------------------------------------
    let mut student1 = StudentObj::new();
    student1.set_gender(Gender::Female);
    student1.set_major(Majors::Literature);
    student1.set_serial();
    println!("composition: {:#?}", student1);
    //----------------
    //struct with tuple
    let s0 = Test(555);
    println!("tuple struct {:#?}", s0);
    //
    let e1 = MyEnum::create_one();
    let e2 = MyEnum::Two((42));
    if let MyEnum::Two(v) = e2 {
        println!("value is {}", v);
    }
}
