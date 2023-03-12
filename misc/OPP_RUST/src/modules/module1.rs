#![allow(dead_code)]
pub mod example1 {
    #[derive(Debug)]
    pub struct Hehe {
        somevar: i32,  //private
        somevar2: i32, //private
        pub pubfield: i32,
    }

    impl Hehe {
        pub fn new(var: i32) -> Self {
            Self {
                somevar: 0,
                somevar2: 0,
                pubfield: var,
            }
        }
        //private method
        fn increase(&mut self) {
            self.somevar += 1;
            self.somevar2 += 1;
        }
        pub fn increase_all(&mut self, newval: i32) {
            self.pubfield += newval;
            self.increase(); //callprivate method
        }
    }
}
pub mod enum_example {
    // enums are public

    #[derive(Debug)]
    pub struct Test(pub i32);
    #[derive(Debug)]
    pub enum MyEnum {
        One(i32),
        Two(i32),
    }
    impl MyEnum {
        pub fn create_one() -> Self {
            Self::One(0)
        }
        //hmmm
        pub fn change_value(&mut self, val: i32) {
            todo!()
        }
    }
}
pub mod example2 {
    #[derive(Debug)]
    pub struct TupleStruct(pub i32, i32, String, pub i32);
    impl TupleStruct {
        pub fn new() -> Self {
            Self(0, 0, String::from("default"), 0)
        }
        pub fn mod_pub(&mut self, v0: i32, v1: i32) {
            self.0 = v0;
            self.3 = v1;
        }
    }
}

pub mod example_composition {

    #[derive(Debug)]
    pub enum Gender {
        Male,
        Female,
    }

    pub trait Human {
        fn set_gender(&mut self, gender: Gender);
        fn set_serial(&mut self);
    }

    #[derive(Debug)]
    pub enum Majors {
        Physic,
        Chemystry,
        Literature,
    }
    // student requires to implement student
    pub trait Student: Human + Serial {
        fn set_major(&mut self, major: Majors);
    }

    #[derive(Debug)]
    pub struct StudentObj {
        major: Majors,
        gender: Gender,
        id: i32,
    }
    impl StudentObj {
        pub fn new() -> Self {
            Self {
                major: Majors::Physic,
                gender: Gender::Male,
                id: 0,
            }
        }
    }
    impl Student for StudentObj {
        fn set_major(&mut self, major: Majors) {
            self.major = major;
        }
    }

    impl Human for StudentObj {
        fn set_gender(&mut self, gender: Gender) {
            self.gender = gender;
        }
        fn set_serial(&mut self) {
            self.id += 1;
        }
    }
    //serial
    pub trait Serial {
        fn assign(&mut self);
    }
    //blanket implementation
    impl<T> Serial for T
    where
        T: Human,
    {
        fn assign(&mut self) {
            self.set_serial()
        }
    }
}
