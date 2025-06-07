use glacier::prelude::*;
use mystu::comment::CreateComment;
use mystu::comment::GetComment;
use mystu::prelude::*;

use mystu::filter::*;
use mystu::post::{CreatePost, GetPost};
use mystu::user::{Login, Refresh, SignUp, VerifyEmail};

///
///
///
///
///
///

// const CORS: &str = "https://aksjfds.github.io";
// const CORS: &str = "http://localhost:3000";
const CORS: &str = "https://mystu.org";
async fn main_router(req: Request) -> Result<Response> {
    let res = match req.uri().path() {
        "/comment/get_comment" => req.filter(Get)?.async_map(GetComment).await,
        "/comment/create_comment" => req.filter(Post)?.async_map(CreateComment).await,
        "/post/get_post" => req.filter(Get)?.async_map(GetPost).await,
        "/post/create_post" => req.filter(Post)?.async_map(CreatePost).await,
        "/user/login" => req.filter(Post)?.async_map(Login).await,
        "/user/verify_email" => req.filter(Get)?.async_map(VerifyEmail).await,
        "/user/sign_up" => req.filter(Post)?.async_map(SignUp).await,
        "/user/refresh" => req.filter(Get)?.async_map(Refresh).await,
        _ => Ok(Response::new().status(404)),
    };

    res
}

async fn router(req: HyperRequest) -> Result<HyperResponse> {
    if req.method() == "OPTIONS" {
        return Response::Ok()
            .header(ACCESS_CONTROL_ALLOW_ORIGIN, CORS)
            .header(ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization")
            .try_into()
            .map_err(|_e| Error::NoCare);
    }

    let req = Request::new(req);
    let res = main_router(req).await;

    let res = match res {
        Ok(res) => res,
        Err(e) => match e {
            // 错误处理
            Error::NoCare => Response::new().status(404),
            Error::Status(code) => Response::new().status(code),
        },
    };

    res.header(ACCESS_CONTROL_ALLOW_ORIGIN, CORS)
        .header(ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization")
        .try_into()
        .map_err(|_e| Error::NoCare)
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use std::net::SocketAddr;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    // let addr = SocketAddr::from(([0, 0, 0, 0], 443));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    glacier::Glacier::bind(addr).serve(router).await.unwrap();

    Ok(())
}
