use glacier::prelude::*;
use mystu::{post::get_post::GetPost, prelude::*};

async fn router(req: HyperRequest) -> Result<HyperResponse> {
    let req = Request::new(req);

    let res = match req.uri().path() {
        "/post/get_post" => req.async_map(GetPost).await,
        _ => todo!(),
    }?;

    res.try_into().map_err(|_e| Error::NoCare)
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use std::net::SocketAddr;

    mystu::database::Postgres::init();

    let addr = SocketAddr::from(([0, 0, 0, 0], 443));
    glacier::client::Glacier::bind(addr)
        .serve(router)
        .await
        .unwrap();

    Ok(())
}
