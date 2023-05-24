use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Mydata {
    pub data: String,
    pub iter: isize,
}
