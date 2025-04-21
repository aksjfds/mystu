use crate::{database::Postgres, prelude::*};
use glacier::prelude::*;
use serde::{Deserialize, Serialize};

use crate::jwt::{self, Key};

///
///
///
///
///
///

/**
    * 成功返回 1,否则返回-1
 */
pub struct SignUp;

impl HandleReq<Error> for SignUp {
    #[tracing::instrument(name = "SignUp", level = "debug", skip(self, req))]
    async fn async_handle(self, mut req: Request) -> Result<Response> {
        // 验证jwt
        let payload: SignUpPayload<String> = req
            .headers()
            .get(AUTHORIZATION)
            .map(|token| jwt::decode(token, Key::access_key()))
            .ok_or_else(|| Error::Status(401))??;

        // 获取参数
        let param = req.body::<SignUpParam>().await.ok_or(Error::NoCare)?;

        // 验证
        if payload.verify_code != param.verify_code || payload.email != param.email {
            return Err(Error::Status(401));
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
