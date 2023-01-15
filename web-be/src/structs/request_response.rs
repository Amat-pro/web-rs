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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SendMailCodeVO {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ArticleCreateAO {
    pub title: String,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ArticleCreateVO {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ArticleUpdateAO {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ArticleUpdateVO {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ArticleSearchAO {
    pub key_word: String,
    pub page_param: Option<crate::structs::PageParam>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ArticleListMineAO {
    pub page_param: Option<crate::structs::PageParam>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub create_time: i64,
    pub update_time: i64,
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

impl SendMailCodeVO {
    pub fn new() -> Self {
        Self {}
    }
}

impl ArticleCreateVO {
    pub fn new() -> Self {
        Self {}
    }
}

impl ArticleUpdateVO {
    pub fn new() -> Self {
        Self {}
    }
}

impl Article {
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            title: "".to_string(),
            description: None,
            summary: None,
            content: None,
            create_time: 0,
            update_time: 0,
        }
    }
}
