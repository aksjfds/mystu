use crate::prelude::*;
use glacier::prelude::*;

use serde::Deserialize;

use crate::database::Postgres;
use crate::jwt::{self, Key};
use crate::tool::{random_code, stu};
use crate::user::SignUpPayload;

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
        let Some("@stu.edu.cn") = email.email.get(email.email.len() - 11..) else {
            return Err(Error::Status(401));
        };

        // 判断是否已注册
        let false = is_exists(&email).await? else {
            return Err(Error::Status(404));
        };

        // 生成验证码
        let verify_code = random_code();

        // 生成 JWT
        let payload = SignUpPayload {
            email: email.email.as_str(),
            verify_code: verify_code.as_str(),
        };
        let jwt = jwt::encode(&payload, Key::sign_up_key(), jwt::SIGN_UP_DURATION)?;

        // 发送stu邮箱
        stu(&email.email, verify_code).map_err(|e| {
            tracing::debug!("{:#?}", e);
            Error::Status(534)
        })?;

        Ok(jwt.into())
    }
}

#[derive(Debug, Deserialize)]
pub struct Email {
    pub email: String,
}

async fn is_exists(email: &Email) -> Result<bool> {
    const SQL: &str = "SELECT EXISTS (SELECT 1 FROM users WHERE email = $1)";

    sqlx::query_scalar(SQL)
        .bind(&email.email)
        .fetch_one(Postgres::pool())
        .await
        .map_err(Into::into)
}
