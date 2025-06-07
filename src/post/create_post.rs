use crate::database::Postgres;
use crate::prelude::*;
use glacier::prelude::*;

use serde::Deserialize;
use serde::Serialize;

use crate::jwt::Key;
use crate::jwt::LoginPayload;

///
///
///
///
///
///

/**
# Error
    * 未携带token - 401
    * token异常 - 401
    * 成功 - 返回 id
    * 失败 - 返回-1
 */
pub struct CreatePost;

impl HandleReq<Error> for CreatePost {
    #[tracing::instrument(name = "CreatePost", level = "debug", skip(self, req))]
    async fn async_handle(self, mut req: Request) -> Result<Response> {
        let payload: LoginPayload = req
            .headers()
            .get(AUTHORIZATION)
            .map(|token| crate::jwt::decode(token, Key::access_key()))
            .ok_or_else(|| Error::Status(401))??;

        let res = req
            .body::<CreatePostParam>()
            .await
            .and_then(|param| match payload.username == param.author {
                true => Some(param),
                false => None,
            })
            .async_map(sql_create_post)
            .await
            .ok_or(Error::Status(401))?
            .map(Into::into);

        res
    }
}

async fn sql_create_post(post: CreatePostParam) -> Result<i32> {
    const SQL: &str = "WITH ins AS ( \
            INSERT INTO posts(author, content) VALUES ($1, $2) RETURNING id \
        )\
        SELECT COALESCE((SELECT id FROM ins), -1)";

    let id = sqlx::query_scalar(SQL)
        .bind(post.author)
        .bind(post.content)
        .fetch_one(Postgres::pool())
        .await
        .map_err(Into::into);
    id
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePostParam {
    pub author: String,
    pub content: String,
}
