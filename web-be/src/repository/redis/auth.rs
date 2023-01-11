use redis::RedisResult;
use tracing::debug;

const AUTH_PREFIX: &'static str = "auth";

pub async fn set_email_code_with_default_expire(code: &String, mail: &String) -> RedisResult<()> {
    debug!(
        "set_email_code_with_default_expire, code: {}, mail: {}",
        code, mail
    );
    let key = build_email_code_key(mail);

    debug!("set_email_code_with_default_expire, key: {}", key);

    crate::lib::redis::set(&key, &code).await?;

    crate::lib::redis::pexpire(&key, 120000 as u64).await
}

fn build_email_code_key(mail: &String) -> String {
    format!(
        "{}:{}:{}{}",
        crate::repository::redis::REDIS_PREFIX,
        self::AUTH_PREFIX,
        "email:code:email-",
        mail
    )
}
