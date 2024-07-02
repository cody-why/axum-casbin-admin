use super::context;
use crate::{error::Result, Error};

const CACHE_KEY_RETRY_NUM: &str = "login_retry_num";
const CACHE_KEY_RETRY_TTL: &str = "login_retry_ttl";

// struct LoginRetryNum;
// struct LoginRetryTTL;

// crate::impl_cache!(LoginRetryNum=>u64, 60*10);
// crate::impl_cache!(LoginRetryTTL=>Instant, 0,
// context().config.login_fail_retry);

///is need to wait
pub async fn is_need_wait_login_ex(account: &str) -> Result<u64> {
    if context().config.login_fail_retry == 0 {
        return Ok(0);
    }
    let num: Option<u64> = context()
        .cache_service
        .get_json(&format!("{}{}", CACHE_KEY_RETRY_NUM, account))
        .await?;
    let num = num.unwrap_or(0);

    // let retry = LoginRetryNum::get_cache(account);
    // if retry.is_none() {
    //     return Ok(0);
    // }
    // let num = retry.unwrap();

    if num >= context().config.login_fail_retry {
        let wait_sec: i64 = context()
            .cache_service
            .ttl(&format!("{}{}", CACHE_KEY_RETRY_TTL, account))
            .await
            .unwrap_or_default();
        // let retry = LoginRetryTTL::get_cache(account);
        // if retry.is_none() {
        //     return Ok(0);
        // }
        // let wait_sec = retry.unwrap().elapsed().as_secs() as i64;
        // let wait_sec = context().config.login_fail_retry_wait_sec as i64 - wait_sec;
        if wait_sec > 0 {
            return Error::err(format!("操作频繁,请等待{{}}秒后重试={}", wait_sec));
        }
    }
    Ok(num)
}

/// Add cache retry record
pub async fn add_retry_login_limit_num(account: &str) -> Result<()> {
    if context().config.login_fail_retry == 0 {
        return Ok(());
    }
    let num: Option<u64> = context()
        .cache_service
        .get_json(&format!("{}{}", CACHE_KEY_RETRY_NUM, account))
        .await?;
    let mut num = num.unwrap_or(0);
    num += 1;
    context()
        .cache_service
        .set_string(&format!("{}{}", CACHE_KEY_RETRY_NUM, account), &num.to_string(), 60 * 10)
        .await?;
    context()
        .cache_service
        .set_string(
            &format!("{}{}", CACHE_KEY_RETRY_TTL, account),
            &num.to_string(),
            context().config.login_fail_retry_wait_sec,
        )
        .await?;
    // let retry = LoginRetryNum::get_cache(account);
    // let retry = match retry {
    //     Some(mut r) => {
    //         r += 1;
    //         r
    //     },
    //     None => 0,
    // };

    // LoginRetryNum::set_cache(account, retry);
    // LoginRetryTTL::set_cache(account, Instant::now());
    Ok(())
}

pub async fn remove_retry_login_limit_num(account: &str) -> Result<()> {
    if context().config.login_fail_retry == 0 {
        return Ok(());
    }
    context()
        .cache_service
        .remove(&format!("{}{}", CACHE_KEY_RETRY_NUM, account))
        .await?;

    // LoginRetryNum::remove_cache(account);

    Ok(())
}
