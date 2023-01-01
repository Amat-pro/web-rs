use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct HelloWorldVO {
    desc: String,
    age: i8,
}

impl HelloWorldVO {
    pub fn new(desc: String, age: i8) -> Self {
        HelloWorldVO {
            desc: desc,
            age: age,
        }
    }
}
