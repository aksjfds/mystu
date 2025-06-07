#![allow(unused)]

use crate::{database::Postgres, prelude::*};
use glacier::prelude::*;
use serde::{Deserialize, Serialize};

pub struct Like;

impl HandleReq<Error> for Like {
    #[tracing::instrument(name = "Like", level = "debug", skip(self, req))]
    async fn async_handle(self, req: Request) -> Result<Response> {
        let res = req
            .param::<LikeParam>()
            .ok_or(Error::NoCare)
            .async_map(like)
            .await?
            .map(Into::into);

        res
    }
}

async fn like(param: LikeParam) -> Result<i32> {
    return Ok(1);
}

#[derive(Deserialize)]
pub struct LikeParam {
    id: i32,
}
