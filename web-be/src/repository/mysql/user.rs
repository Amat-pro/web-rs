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
    pub fn new() -> Self {
        Self {
            id: 0,
            nick_name: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            is_delete: 0,
            create_time: 0,
            update_time: 0,
        }
    }

    pub async fn create(
        nick_name: String,
        email: String,
        password: String,
    ) -> Result<UserEntity, Error> {
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
            Ok(ok) => {
                let mut u = UserEntity::new();
                u.id = ok.last_insert_id();
                u.nick_name = nick_name;
                u.email = email;
                u.password = password;
                u.create_time = timestamp_millis;
                u.update_time = timestamp_millis;
                Ok(u)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn find_user_by_email(email: &String) -> Result<UserEntity, Error> {
        let mysql_pool = crate::lib::MYSQL_POOL.clone();

        let query_r = sqlx::query_as!(
            UserEntity,
            "SELECT * FROM `user` where email = ? AND is_delete = 0 LIMIT 1",
            email
        )
        .fetch_one(&mysql_pool)
        .await;

        match query_r {
            Ok(entity) => Ok(entity),
            Err(e) => Err(e),
        }
    }

    pub async fn find_one_user_by_email_or_nick_name(
        email: &String,
        nick_name: &String,
    ) -> Result<UserEntity, Error> {
        let mysql_pool = crate::lib::MYSQL_POOL.clone();

        let query_r = sqlx::query_as!(
            UserEntity,
            "SELECT * from `user` where email = ? or nick_name = ? LIMIT 1",
            email,
            nick_name
        )
        .fetch_one(&mysql_pool)
        .await;

        match query_r {
            Ok(entity) => Ok(entity),
            Err(e) => Err(e),
        }
    }

    pub async fn update_pass_by_email(email: &String, pass: &String) -> Result<(), Error> {
        let mysql_pool = crate::lib::MYSQL_POOL.clone();

        let timestamp_millis = chrono::Local::now().timestamp_millis();
        let update_r = sqlx::query!(
            r#"
            UPDATE `user` SET password = ?, update_time = ? WHERE email = ?
            "#,
            pass,
            timestamp_millis,
            email,
        )
        .execute(&mysql_pool)
        .await;

        match update_r {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UserEntity;
    use tokio::runtime;

    #[test]
    fn test_find_user_by_email() {
        let rt = runtime::Runtime::new().unwrap();
        let find_r = rt.block_on(UserEntity::find_user_by_email(
            &"2892798998@qq.com".to_string(),
        ));
        match find_r {
            Ok(user) => {
                println!("user: {:?}", user);
            }
            Err(err) => {
                println!("err: {}", err);
            }
        }
    }

    #[test]
    fn test_find_one_user_by_email_or_nick_name() {
        let rt = runtime::Runtime::new().unwrap();
        let find_r = rt.block_on(UserEntity::find_one_user_by_email_or_nick_name(
            &"2892798998@qq.com".to_string(),
            &"maomao".to_string(),
        ));
        match find_r {
            Ok(entity) => {
                println!("entity: {:?}", entity);
            }
            Err(err) => {
                println!("err: {}", err);
            }
        }
    }
}
