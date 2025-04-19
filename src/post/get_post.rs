use crate::{database::Postgres, prelude::*};
use glacier::{prelude::*, resultext::AsyncMapExt};
use serde::{Deserialize, Serialize};

pub struct GetPost;

impl HandleReq<Error> for GetPost {
    async fn async_handle(self, req: Request) -> Result<Response> {
        let res = req
            .param::<GetPostParam>()
            .ok_or(Error::NoCare)
            .async_map(get_post)
            .await
            .map(Into::into);

        res
    }
}

pub async fn get_post(param: GetPostParam) -> Vec<Post> {
    const SQL: &str = "SELECT id, title, author, content, \
        to_char(time, 'YYYY-MM-DD HH24:MI') AS time \
        FROM posts WHERE id > $1 LIMIT $2";

    sqlx::query_as(SQL)
        .bind(param.last_id)
        .bind(5)
        .fetch_all(Postgres::pool())
        .await
        .unwrap()
}

#[derive(Deserialize)]
pub struct GetPostParam {
    pub last_id: i32,
    pub limit: i16,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub time: String,
    pub content: String,
}

#[tokio::test]
async fn test() {
    let param = GetPostParam {
        last_id: 0,
        limit: 5,
    };

    let posts = get_post(param).await;
    println!("{:#?}", posts);
}
