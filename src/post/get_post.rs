use crate::{database::Postgres, prelude::*};
use glacier::prelude::*;
use serde::{Deserialize, Serialize};

///
///
///
///
///
///

pub struct GetPost;

impl HandleReq<Error> for GetPost {
    #[tracing::instrument(name = "GetPost", level = "debug", skip(self, req))]
    async fn async_handle(self, req: Request) -> Result<Response> {
        let res = req
            .param::<GetPostParam>()
            .ok_or(Error::NoCare)
            .async_map(get_post)
            .await?
            .map(Into::into);

        res
    }
}

pub async fn get_post(param: GetPostParam) -> Result<Vec<Post>> {
    const SQL: &str = "SELECT id, author, content, \
        to_char(time, 'YYYY-MM-DD HH24:MI') AS time \
        FROM posts WHERE id > $1 LIMIT $2";

    let posts = sqlx::query_as(SQL)
        .bind(param.last_id)
        .bind(5)
        .fetch_all(Postgres::pool())
        .await
        .map_err(Into::into);

    posts
}

#[derive(Debug, Deserialize)]
pub struct GetPostParam {
    last_id: i32,
    #[allow(dead_code)]
    limit: i16,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: i32,
    pub author: String,
    pub time: String,
    pub content: String,
}
