use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SendMailCodeAO {
    to: String,
    // register, change-pass
    mail_type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct RegisterAO {
    pub nick_name: String,
    pub email: String,
    pub password: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct RegisterVO {
    pub id: String,
    pub nick_name: String,
    pub email: String,

    pub access_token: String,
    pub token_type: String,
    pub expire_time: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct LoginAO {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct LoginVO {
    pub id: String,
    pub nick_name: String,
    pub email: String,

    pub access_token: String,
    pub token_type: String,
    pub expire_time: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PassChangeAO {
    pub email: String,
    pub new_pass: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PassChangeVO {}

impl SendMailCodeAO {
    pub fn get_to(&self) -> String {
        self.to.clone()
    }
    pub fn get_mail_type(&self) -> String {
        self.mail_type.clone()
    }
}

impl RegisterVO {
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            nick_name: "".to_string(),
            email: "".to_string(),

            access_token: "".to_string(),
            token_type: "".to_string(),
            expire_time: 0,
        }
    }
}

impl LoginVO {
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            nick_name: "".to_string(),
            email: "".to_string(),

            access_token: "".to_string(),
            token_type: "".to_string(),
            expire_time: 0,
        }
    }
}

impl PassChangeVO {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SendMailCodeVO {}

impl SendMailCodeVO {
    pub fn new() -> Self {
        Self {}
    }
}
