use serde::{Deserialize, Serialize};
use templates::sender::Operation;
#[derive(Serialize, Deserialize, Debug)]
pub struct Playgame();

#[typetag::serde]
impl Operation for Playgame {
    fn extract_operation(&self) {}
}
