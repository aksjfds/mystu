use crate::{
    database::{Postgres, Redis},
    prelude::*,
};
use glacier::prelude::*;
use redis::Commands;
use serde::{Deserialize, Serialize};

use super::verify_email::is_exists;

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
    * 验证码错误 - 703
 */
pub struct SignUp;

impl HandleReq<Error> for SignUp {
    #[tracing::instrument(name = "SignUp", level = "debug", skip(self, req))]
    async fn async_handle(self, mut req: Request) -> Result<Response> {
        // 获取参数
        let param = req.body::<SignUpParam>().await.ok_or(Error::NoCare)?;

        // 判断后缀 @stu.edu.cn
        let Some("@stu.edu.cn") = param.email.get(param.email.len() - 11..) else {
            return Err(Error::Status(701));
        };

        // 判断是否已注册
        let false = is_exists(&param.email).await? else {
            return Err(Error::Status(702));
        };

        // 验证
        let verify_code: Option<String> = Redis::get_conn()?.get(param.email.as_str())?;
        match verify_code {
            Some(verify_code) => {
                if verify_code != param.verify_code {
                    return Err(Error::Status(703));
                }
            }
            None => return Err(Error::Status(703)),
        }

        // 注册
        let res = sql_sign_up(&param.email, &param.username, &param.password)
            .await
            .map(Into::into);

        res
    }
}

async fn sql_sign_up(email: &str, username: &str, password: &str) -> Result<i32> {
    const SQL: &str = "WITH ins AS ( \
                INSERT INTO users (email, username, password) VALUES ($1, $2, $3) \
                ON CONFLICT (username) DO NOTHING \
                RETURNING 1 AS res\
            ) \
            SELECT COALESCE((SELECT res FROM ins), -1)";

    let id = sqlx::query_scalar(SQL)
        .bind(email)
        .bind(username)
        .bind(password)
        .fetch_one(Postgres::pool())
        .await
        .map_err(Into::into);

    id
}

#[derive(Debug, Deserialize, sqlx::FromRow)]
struct SignUpParam {
    email: String,
    username: String,
    password: String,
    verify_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignUpPayload<T> {
    pub email: T,
    pub verify_code: T,
}
