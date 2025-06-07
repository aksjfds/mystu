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
pub struct CreateComment;

impl HandleReq<Error> for CreateComment {
    #[tracing::instrument(name = "CreateComment", level = "debug", skip(self, req))]
    async fn async_handle(self, mut req: Request) -> Result<Response> {
        let payload: LoginPayload = req
            .headers()
            .get(AUTHORIZATION)
            .map(|token| crate::jwt::decode(token, Key::access_key()))
            .ok_or_else(|| Error::Status(401))??;

        let res = req
            .body::<CreateCommentParam>()
            .await
            .and_then(|param| match payload.username == param.username {
                true => Some(param),
                false => None,
            })
            .async_map(sql_create_comment)
            .await
            .ok_or(Error::Status(401))?
            .map(Into::into);

        res
    }
}

async fn sql_create_comment(comment: CreateCommentParam) -> Result<i32> {
    const SQL1: &str = "\
INSERT INTO comments (post_id, parent_id, reply_to, username, content) VALUES ($1, $2, $3, $4, $5) \
RETURNING comment_id";
    const SQL2: &str = "\
UPDATE comments SET reply_count = reply_count + 1 WHERE EXISTS (SELECT $1) AND post_id = $2 AND comment_id = $1\
";

    let mut tx = Postgres::pool().begin().await?;

    let comment_id: i32 = sqlx::query_scalar(SQL1)
        .bind(comment.post_id)
        .bind(comment.parent_id)
        .bind(comment.reply_to)
        .bind(comment.username)
        .bind(comment.content)
        .fetch_one(&mut *tx)
        .await?;

    sqlx::query(SQL2)
        .bind(comment.parent_id)
        .bind(comment.post_id)
        .execute(&mut *tx)
        .await
        .inspect_err(|e| println!("{:#?}", e))?;

    tx.commit().await?;

    Ok(comment_id)
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CreateCommentParam {
    pub post_id: i32,
    pub parent_id: Option<i32>,
    pub reply_to: Option<String>,
    pub username: String,
    pub content: String,
}
