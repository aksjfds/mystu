use crate::{
    database::Redis,
    jwt::{self, Key, LoginPayload},
    prelude::*,
};
use glacier::prelude::*;
use redis::Commands;

use super::login::Token;

///
///
///
///
///
///

/**
# Error
    * 未携带长token - 401
    * 长token验证失败 - 401
    * 长token过期 - 701

 */
pub struct Refresh;

impl HandleReq<Error> for Refresh {
    #[tracing::instrument(name = "Refresh", level = "debug", skip(self, req))]
    async fn async_handle(self, req: Request) -> Result<Response> {
        // 获取长token
        let refresh_token = req.headers().get(AUTHORIZATION).ok_or(Error::Status(401))?;

        // 验证长token
        let payload = jwt::decode::<LoginPayload>(refresh_token, Key::refresh_key())?;

        // 生成键名，在redis中判断是否存在
        let refresh_token = std::str::from_utf8(refresh_token.as_ref()).map_err(|e| {
            tracing::debug!("{:#?}", e);
            Error::Status(401)
        })?;
        let len = refresh_token.len();
        let status = match len > 16 {
            true => &refresh_token[len - 16..],
            false => &refresh_token,
        };
        Redis::get_conn()?
            .get_del::<_, Option<u8>>(status)?
            .ok_or_else(|| Error::Status(701))?;

        // 存在的话，生成长短token
        let refresh_token = jwt::encode(&payload, Key::refresh_key(), jwt::REFRESH_DURATION)?;
        let access_token = jwt::encode(&payload, Key::access_key(), jwt::ACCESS_DURATION)?;
        
        let len = refresh_token.len();
        let status = match len > 16 {
            true => &refresh_token[len - 16..],
            false => &refresh_token,
        };
        let _: () = Redis::get_conn()?.set_ex(status, 0u8, jwt::REFRESH_DURATION.as_secs())?;

        // 返回长短token
        let res = Ok(Token {
            refresh_token,
            access_token,
        })
        .map(Into::into);

        res
    }
}