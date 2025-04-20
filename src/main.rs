use glacier::prelude::*;
use mystu::prelude::*;

use mystu::post::{CreatePost, GetPost};
use mystu::user::{Login, SignUp};

///
/// 
/// 
/// 
/// 
/// 

async fn router(req: HyperRequest) -> Result<HyperResponse> {
    let req = Request::new(req);

    let res = match req.uri().path() {
        "/post/get_post" => req.async_map(GetPost).await,
        "/post/create_post" => req.async_map(CreatePost).await,
        "/user/login" => req.async_map(Login).await,
        "/user/signup" => req.async_map(SignUp).await,
        _ => Ok(Response::new().status(404)),
    };

    let res = match res {
        Ok(res) => res,
        Err(e) => match e {
            // 错误处理
            Error::NoCare => Response::new().status(404),
            Error::Status(code) => Response::new().status(code),
        },
    };

    res.try_into().map_err(|_e| Error::NoCare)
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use std::net::SocketAddr;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    let addr = SocketAddr::from(([0, 0, 0, 0], 443));
    glacier::Glacier::bind(addr).serve(router).await.unwrap();

    Ok(())
}
