use num::Num;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Argument<A: Num> {
    num: Option<A>,
    string: Option<String>,
}
impl<A: Num> Argument<A> {
    pub fn assign_num(mut self, arg: A) -> Self {
        self.num = Some(arg);
        self
    }
    pub fn assign_string(mut self, string: String) -> Self {
        self.string = Some(string);
        self
    }
    pub fn finallize(self) -> Self {
        self
    }
}
/*
#[derive(Serialize, Deserialize, Debug)]
pub struct Argumentz(Vec<Box<dyn Toargs>>);

#[typetag::serde(tag = "type")]
pub trait Toargs: ArgumentsGetter + ArgumentsSetter + std::fmt::Debug {}

#[typetag::serde(tag = "type")]
pub trait ArgumentsGetter {
    fn get_arg_id(&self) -> isize;
    fn get_arg_data(&self) -> BTreeMap<usize, isize>;
}

#[typetag::serde(tag = "type")]
pub trait ArgumentsSetter {
    fn set_arg_id(&mut self, id: isize);
    fn set_arg_data(&mut self, arg: BTreeMap<usize, isize>);
}

pub trait ArgIface {
    fn set();
    fn get() -> Self;
}
*/
