use num::Num;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Argument<A: Num + Copy> {
    num: Option<A>,
    string: Option<String>,
}
impl<A: Num + Copy> Argument<A> {
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
    pub fn try_get_num(&self) -> Option<A> {
        self.num.clone()
    }
    pub fn try_get_string(&self) -> Option<String> {
        self.string.to_owned()
    }
}
