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

pub struct CreatePost;

impl HandleReq<Error> for CreatePost {
    #[tracing::instrument(name = "create_post", level = "debug", skip(self, req))]
    async fn async_handle(self, mut req: Request) -> Result<Response> {
        let payload: LoginPayload = req
            .headers()
            .get(AUTHORIZATION)
            .map(|token| crate::jwt::decode(token, Key::access_key()))
            .ok_or_else(|| Error::Status(401))??;

        let body = req
            .body::<CreatePostParam>()
            .await
            .and_then(|param| match payload.username == param.author {
                true => Some(param),
                false => None,
            })
            .async_map(sql_create_post)
            .await
            .map(Into::into)
            .ok_or(Error::NoCare);

        body
    }
}

async fn sql_create_post<'a, 'b, 'c>(post: CreatePostParam<'a, 'b, 'c>) -> i32 {
    const SQL: &str = "WITH ins AS ( \
            INSERT INTO posts(title, author, content) VALUES ($1, $2, $3) \
            ON CONFLICT (title) DO NOTHING \
            RETURNING id \
        )\
        SELECT COALESCE((SELECT id FROM ins), -1)";

    sqlx::query_scalar(SQL)
        .bind(post.title)
        .bind(post.author)
        .bind(post.content)
        .fetch_one(Postgres::pool())
        .await
        .unwrap_or(0)
}

use std::borrow::Cow;
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePostParam<'a, 'b, 'c> {
    pub title: Cow<'a, str>,
    pub author: Cow<'b, str>,
    pub content: Cow<'c, str>,
}
