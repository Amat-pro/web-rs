use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SendMailCodeAO {
    to: String,
}

impl SendMailCodeAO {
    pub fn get_to(&self) -> String {
        self.to.clone()
    }
}
