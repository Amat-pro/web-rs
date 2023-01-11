use serde::{Deserialize, Serialize};
use sqlx::Error;

// const TABLE_NAME_USER: &'static str = "user";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEntity {
    pub id: u64,
    pub nick_name: String,
    pub email: String,
    pub password: String,
    pub is_delete: i8,
    pub create_time: i64,
    pub update_time: i64,
}

impl UserEntity {
    pub async fn create(nick_name: String, email: String, password: String) -> Result<(), Error> {
        let mysql_pool = crate::lib::MYSQL_POOL.clone();

        let timestamp_millis = chrono::Local::now().timestamp_millis();
        let insert_r = sqlx::query!(
            r#"
            INSERT INTO `user` (nick_name, email, password, is_delete, create_time, update_time) VALUES (?, ?, ?, ?, ?, ?)
            "#, 
            nick_name, email, password, 0, timestamp_millis, timestamp_millis)
        .execute(&mysql_pool)
        .await;

        match insert_r {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn find_pass_by_email(email: String) -> Result<String, Error> {
        let mysql_pool = crate::lib::MYSQL_POOL.clone();

        let query_r = sqlx::query_as!(
            UserEntity,
            "SELECT * FROM `user` where email = ? AND is_delete = 0",
            email
        )
        .fetch_one(&mysql_pool)
        .await;

        match query_r {
            Ok(entity) => Ok(entity.password.clone()),
            Err(e) => Err(e),
        }
    }
}
