use crate::config::CONFIG;
use crate::repository::mysql::UserEntity;
use crate::structs::{global_response, Claims, RegisterAO, RegisterVO, SendMailCodeVO, UserInfo};
use axum::extract::Json;
use serde_json::Value;
use tracing::{info, warn};

// #[tracing::instrument]
pub async fn send_email_with_default_limit(to: &String) -> Json<Value> {
    let key = format!(
        "{}:{}:email-to:{}",
        crate::lib::redis::REDIS_PREFIX,
        crate::lib::redis::AUTH_PREFIX,
        to
    );

    let set_r = crate::lib::redis::set_nx_with_secs_expire(&key, &"1".to_string(), 120).await;

    if set_r.is_err() {
        warn!(
            "send_email_with_default_limit, check default limit use redis err: {:?}",
            set_r.err().unwrap()
        );
        return global_response::new(
            global_response::ERROR_CODE_EMAIL_SEND_ERROR,
            SendMailCodeVO::new(),
        );
    }

    match set_r.unwrap() {
        redis::Value::Okay => {
            let code = crate::utils::rand::generate_numbers(6);

            let key = generate_email_code_key(to);

            let set_r = crate::lib::redis::set_with_secs_expire(&key, &code, 5 * 60).await;
            if set_r.is_err() {
                warn!(
                    "send_email_with_default_limit, save expire code to redis err: {:?}",
                    set_r.err().unwrap()
                );
                return global_response::new(
                    global_response::ERROR_CODE_EMAIL_SEND_ERROR,
                    SendMailCodeVO::new(),
                );
            }

            let body = format!("邮箱验证码: {}", code);
            let send_r =
                crate::lib::send_mail(to.to_string(), "WEB-RS 发送邮箱验证码".to_string(), body);
            if send_r.is_err() {
                warn!(
                    "send_email_with_default_limit, send mail err: {:?}",
                    send_r.err().unwrap()
                );
                return global_response::new(
                    global_response::ERROR_CODE_EMAIL_SEND_ERROR,
                    SendMailCodeVO::new(),
                );
            }

            global_response::new(global_response::ERROR_CODE_SUCCESS, SendMailCodeVO::new())
        }
        _ => {
            info!("send_email_with_default_limit, check default limit use redis invalid");
            global_response::new(
                global_response::ERROR_CODE_EMAIL_SEND_INVALID,
                SendMailCodeVO::new(),
            )
        }
    }
}

#[tracing::instrument]
pub async fn register_user(register_info: &RegisterAO) -> Json<Value> {
    // check code
    let check_code_r = check_email_code(&register_info.email, &register_info.code).await;
    if check_code_r.is_err() {
        warn!(
            "register_user, check code err: {}",
            check_code_r.err().unwrap()
        );
        return global_response::new(global_response::ERROR_CODE_SERVER_ERROR, RegisterVO::new());
    }

    if !check_code_r.unwrap() {
        return global_response::new(
            global_response::ERROR_CODE_EMAIL_CODE_INVALID,
            RegisterVO::new(),
        );
    }

    // check nick_name, email repeated
    let exist_user_r = UserEntity::find_one_user_by_email_or_nick_name(
        &register_info.email,
        &register_info.nick_name,
    )
    .await;

    if exist_user_r.is_ok() {
        let exist_user = exist_user_r.unwrap();
        if exist_user.email == register_info.email {
            return global_response::new(
                global_response::ERROR_CODE_REGISTER_ERR_EMAIL_REPEATED,
                RegisterVO::new(),
            );
        }
        return global_response::new(
            global_response::ERROR_CODE_REGISTER_ERR_NICKNAME_REPEATED,
            RegisterVO::new(),
        );
    }

    let err = exist_user_r.err().unwrap();
    match err {
        sqlx::Error::RowNotFound => {
            // create user
            let create_r = UserEntity::create(
                register_info.nick_name.clone(),
                register_info.email.clone(),
                register_info.password.clone(),
            )
            .await;
            if create_r.is_err() {
                warn!(
                    "register_user, create user err: {}",
                    create_r.err().unwrap()
                );
                return global_response::new(
                    global_response::ERROR_CODE_SERVER_ERROR,
                    RegisterVO::new(),
                );
            }

            let entity = create_r.unwrap();

            // token
            let mut u = UserInfo::new();
            let UserEntity {
                id,
                email,
                nick_name,
                ..
            } = entity;
            u.id = id.to_string();
            u.email = email.clone();
            u.nick_name = nick_name.clone();

            let expire = chrono::Local::now().timestamp_millis() as u64
                + CONFIG
                    .clone()
                    .get_security_config()
                    .get_jwt_config()
                    .get_exp();

            let token = crate::utils::jwt::generate_token(&Claims::new(expire, u)).unwrap();

            let mut vo = RegisterVO::new();
            vo.id = id.to_string();
            vo.email = email.clone();
            vo.nick_name = nick_name.clone();

            vo.token_type = "Bearer".to_string();
            vo.access_token = token;
            vo.expire_time = expire;

            global_response::new(global_response::ERROR_CODE_SUCCESS, vo)
        }
        _others => {
            warn!(
                "register_user, check email or nick name repeated err: {}",
                _others
            );
            global_response::new(global_response::ERROR_CODE_SERVER_ERROR, RegisterVO::new())
        }
    }
}

fn generate_email_code_key(to: &String) -> String {
    format!(
        "{}:{}:email-to:{}:code",
        crate::lib::redis::REDIS_PREFIX,
        crate::lib::redis::AUTH_PREFIX,
        to
    )
}

async fn check_email_code(email: &String, code: &String) -> Result<bool, String> {
    let key = generate_email_code_key(email);

    let get_r = crate::lib::redis::get(&key).await;

    if get_r.is_err() {
        return Err(get_r.err().unwrap().to_string());
    }

    let result = get_r.unwrap();
    match result {
        redis::Value::Nil => Ok(false),
        redis::Value::Data(data) => {
            let code_vec = code.as_bytes().to_vec();
            if code_vec.eq(&data) {
                return Ok(true);
            }
            Ok(false)
        }
        _ => Ok(false),
    }
}
