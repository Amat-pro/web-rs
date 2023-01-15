use crate::structs::{build_limit_offset, new_page, Page, PageParam};
use serde::{Deserialize, Serialize};
use sqlx::Error;

// const TABLE_NAME_USER_ARTICLE: &'static str = "user_article";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserArticleEntity {
    pub id: u64,
    pub article_id: String,
    pub user_id: i64,
    pub is_delete: i8,
    pub create_time: i64,
    pub update_time: i64,
}

impl UserArticleEntity {
    pub fn new() -> Self {
        Self {
            id: 0,
            article_id: "".to_string(),
            user_id: 0,
            is_delete: 0,
            create_time: 0,
            update_time: 0,
        }
    }

    pub async fn create(article_id: &String, user_id: i64) -> Result<UserArticleEntity, Error> {
        let mysql_pool = crate::lib::MYSQL_POOL.clone();

        let timestamp_millis = chrono::Local::now().timestamp_millis();
        let insert_r = sqlx::query!(
            r#"
            INSERT INTO `user_article` (article_id, user_id, is_delete, create_time, update_time) VALUES (?, ?, ?, ?, ?)
            "#,
            article_id, user_id, 0, timestamp_millis, timestamp_millis)
        .execute(&mysql_pool)
        .await;

        match insert_r {
            Ok(ok) => {
                let mut u = UserArticleEntity::new();
                u.id = ok.last_insert_id();
                u.article_id = article_id.clone();
                u.user_id = user_id;
                u.create_time = timestamp_millis;
                u.update_time = timestamp_millis;
                Ok(u)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn find_page_by_user_id(
        user_id: i64,
        page_param: PageParam,
    ) -> Result<Page<Vec<UserArticleEntity>>, Error> {
        let mysql_pool = crate::lib::MYSQL_POOL.clone();

        let count_r = sqlx::query!(
            "SELECT COUNT(*) AS count FROM `user_article` WHERE user_id = ? and is_delete = 0",
            user_id
        )
        .fetch_optional(&mysql_pool)
        .await;

        let count: u64;
        match count_r {
            Err(e) => {
                return Err(e);
            }
            Ok(record) => {
                count = record.unwrap().count as u64;
            }
        }

        if count <= 0 {
            return Ok(new_page(page_param.size, page_param.current, 0, vec![]));
        }

        let limit_offset = build_limit_offset(count, &page_param).unwrap();
        let mut total = count / page_param.size;
        if count % page_param.size > 0 {
            total += 1;
        }

        let select_r = sqlx::query_as!(
            UserArticleEntity,
            r#"
            SELECT * from `user_article` WHERE user_id = ? and is_delete = 0 ORDER BY update_time DESC LIMIT ? OFFSET ?
            "#,
            user_id, limit_offset.limit, limit_offset.offset)
        .fetch_all(&mysql_pool)
        .await;

        match select_r {
            Err(e) => Err(e),
            Ok(records) => Ok(new_page(
                page_param.size,
                page_param.current,
                total,
                records,
            )),
        }
    }
}
