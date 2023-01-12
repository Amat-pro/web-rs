use crate::structs::Claims;
use axum::extract::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// todo router
pub async fn create_article_handler(_claims: Claims, Json(_payload): Json<Value>) -> Json<Value> {
    crate::structs::global_response::new(
        crate::structs::global_response::ERROR_CODE_SUCCESS,
        ArticleCreateVO::new(),
    )
}

// todo router
pub async fn search_article_handler(Json(_payload): Json<Value>) -> Json<Value> {
    let article_items: Vec<ArticleItem> = vec![];

    crate::structs::global_response::new(
        crate::structs::global_response::ERROR_CODE_SUCCESS,
        crate::structs::new_page::<Vec<ArticleItem>>(0, 0, 0, article_items),
    )
}

// todo router
pub async fn list_mine_article_handler(
    _claims: Claims,
    Json(_payload): Json<Value>,
) -> Json<Value> {
    let article_items: Vec<ArticleItem> = vec![];

    crate::structs::global_response::new(
        crate::structs::global_response::ERROR_CODE_SUCCESS,
        crate::structs::new_page::<Vec<ArticleItem>>(0, 0, 0, article_items),
    )
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct ArticleCreateAO {
    title: String,
    description: Option<String>,
    summary: Option<String>,
    content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct ArticleSearchAO {
    key_word: Option<String>,
    page_param: Option<crate::structs::PageParam>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct ArticleListMineAO {
    page_param: Option<crate::structs::PageParam>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ArticleItem {
    title: String,
    description: Option<String>,
    summary: Option<String>,
    content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct ArticleCreateVO {}

impl ArticleCreateVO {
    fn new() -> Self {
        Self {}
    }
}
