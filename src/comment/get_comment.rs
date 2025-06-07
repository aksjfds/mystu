#![allow(unused)]
use crate::{database::Postgres, prelude::*};
use glacier::prelude::*;
use serde::{Deserialize, Serialize};
///
///
///
///
///
///

pub struct GetComment;

impl HandleReq<Error> for GetComment {
    #[tracing::instrument(name = "GetComment", level = "debug", skip(self, req))]
    async fn async_handle(self, req: Request) -> Result<Response> {
        let res = req
            .param::<GetCommentParam>()
            .ok_or(Error::NoCare)
            .async_map(get_comment)
            .await?
            .map(Into::into);

        res
    }
}

pub async fn get_comment(param: GetCommentParam) -> Result<Vec<Comment>> {
    match param.parent_id {
        Some(_) => get_two_level_comment(param).await,
        None => get_one_level_comment(param).await,
    }
}

pub async fn get_one_level_comment(param: GetCommentParam) -> Result<Vec<Comment>> {
    const SQL: &str = "\
SELECT comment_id, parent_id, reply_to, username, content, reply_count, to_char(time, 'YYYY-MM-DD HH24:MI') AS time FROM comments WHERE post_id = $1 AND parent_id IS NULL AND comment_id > $2 LIMIT 5\
";

    let comments = sqlx::query_as(SQL)
        .bind(param.post_id)
        .bind(param.last_id)
        .fetch_all(Postgres::pool())
        .await
        .unwrap();

    Ok(comments)
}

pub async fn get_two_level_comment(param: GetCommentParam) -> Result<Vec<Comment>> {
    const SQL: &str = "\
SELECT comment_id, parent_id, reply_to, username, content, reply_count, to_char(time, 'YYYY-MM-DD HH24:MI') AS time FROM comments WHERE post_id = $1 AND parent_id = $2 AND comment_id > $3 LIMIT 5\
";

    let comments = sqlx::query_as(SQL)
        .bind(param.post_id)
        .bind(param.parent_id)
        .bind(param.last_id)
        .fetch_all(Postgres::pool())
        .await
        .unwrap();

    Ok(comments)
}

#[derive(Debug, Deserialize)]
pub struct GetCommentParam {
    post_id: i32,
    parent_id: Option<i32>,
    last_id: i32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Comment {
    pub comment_id: i32,
    pub parent_id: Option<i32>,
    pub reply_to: Option<String>,
    pub username: String,
    pub content: String,
    pub reply_count: Option<i32>,
    pub time: String,
}
