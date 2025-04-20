use std::borrow::Cow;

use crate::prelude::*;
use glacier::prelude::*;

use crate::database::{Postgres, Redis};
use crate::jwt::{self, LoginPayload};

use redis::Commands;
use serde::{Deserialize, Serialize};

///
///
///
///
///
///

/**
# Error
    * 参数错误 - NoCare
    * 不存在用户 - 401
    * 加密token错误 - 500
    * Redis请求错误 - 500
 */
pub struct Login;

impl HandleReq<Error> for Login {
    #[tracing::instrument(name = "login", level = "debug", skip(self, req))]
    async fn async_handle(self, mut req: Request) -> Result<Response> {
        // 获取邮箱密码
        let user = req.body::<User>().await.ok_or(Error::NoCare)?;

        // 判断是否存在该用户
        let payload = is_exists(user).await.ok_or(Error::Status(401))?;

        // 生成长短token,保存在 localStorage
        let refresh_token = jwt::encode(&payload, jwt::Key::refresh_key(), jwt::REFRESH_DURATION)?;
        let access_token = jwt::encode(&payload, jwt::Key::access_key(), jwt::ACCESS_DURATION)?;

        // 根据长token生成一个键名存在redis（不需要值也行）
        let len = refresh_token.len();
        let status = match len > 16 {
            true => &refresh_token[len - 16..],
            false => &refresh_token,
        };
        let _: () = Redis::get_conn()?
            .set_ex(status, 0u8, jwt::REFRESH_DURATION.as_secs())
            .map_err(|e| {
                tracing::debug!("{:#?}", e);
                Error::Status(500)
            })?;

        // 返回长短token
        let res = Ok(Token {
            refresh_token,
            access_token,
        })
        .map(Into::into);

        res
    }
}

#[derive(Debug, Deserialize)]
struct User<'a, 'b> {
    email: Cow<'a, str>,
    password: Cow<'b, str>,
}

#[derive(Serialize)]
pub(super) struct Token<T> {
    pub refresh_token: T,
    pub access_token: T,
}

async fn is_exists<'a, 'b>(user: User<'a, 'b>) -> Option<LoginPayload> {
    const SQL: &str = "SELECT email, username, role FROM users WHERE email = $1 AND active = TRUE AND password = $2";

    let payload = sqlx::query_as(SQL)
        .bind(user.email)
        .bind(user.password)
        .fetch_optional(Postgres::pool())
        .await
        .map_err(|e| tracing::debug!("{:#?}", e))
        .ok()?;

    payload
}
