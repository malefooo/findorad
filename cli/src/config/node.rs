use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub address: String,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            address: String::from("http://localhost:25576"),
        }
    }
}
