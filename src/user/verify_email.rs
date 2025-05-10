use crate::prelude::*;
use glacier::prelude::*;

use redis::Commands;
use serde::Deserialize;

use crate::database::{Postgres, Redis};
use crate::tool::{random_code, stu};

///
///
///
///
///
///

/**
# Error
    * 邮箱后缀不对 - 701
    * 已注册 - 702
    * 发送邮箱错误 - 534
 */
pub struct VerifyEmail;

impl HandleReq<Error> for VerifyEmail {
    #[tracing::instrument(name = "VerifyEmail", level = "debug", skip(self, req))]
    async fn async_handle(self, req: Request) -> Result<Response> {
        let email = req.param::<Email>().ok_or(Error::NoCare)?;

        // 判断后缀 @stu.edu.cn
        if email.email.len() < 15 {
            return Err(Error::Status(401));
        }
        let Some("@stu.edu.cn") = email.email.get(email.email.len() - 11..) else {
            return Err(Error::Status(701));
        };

        // 判断是否已注册
        let false = is_exists(&email.email).await? else {
            return Err(Error::Status(702));
        };

        // 生成验证码
        let verify_code = random_code();

        // 存入redis 5min
        let _: () =
            Redis::get_conn()?.set_ex(verify_code.as_str(), email.email.as_str(), 60 * 5)?;

        // 发送stu邮箱
        stu(&email.email, verify_code).map_err(|e| {
            tracing::debug!("{:#?}", e);
            Error::Status(534)
        })?;

        Ok(().into())
    }
}

#[derive(Debug, Deserialize)]
pub struct Email {
    pub email: String,
}

pub async fn is_exists(email: &String) -> Result<bool> {
    const SQL: &str = "SELECT EXISTS (SELECT 1 FROM users WHERE email = $1)";

    sqlx::query_scalar(SQL)
        .bind(email)
        .fetch_one(Postgres::pool())
        .await
        .map_err(Into::into)
}
