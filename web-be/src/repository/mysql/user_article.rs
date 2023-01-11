use serde::{Deserialize, Serialize};

// const TABLE_NAME_USER_ARTICLE: &'static str = "user_article";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserArticleEntity {
    pub id: i64,
    pub article_id: i64,
    pub user_id: i64,
    pub is_delete: i8,
    pub create_time: i64,
    pub update_time: i64,
}

impl UserArticleEntity {}
