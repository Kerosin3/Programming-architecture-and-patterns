use serde::{Deserialize, Serialize};

pub trait Arguments {
    fn extract(&self);
}
