use sqlx::postgres::PgPoolOptions;
use std::sync::LazyLock;

///
///
///
///
///
///

pub static mut POOL: LazyLock<sqlx::Pool<sqlx::Postgres>> = LazyLock::new(|| {
    #[cfg(feature = "dotenv")]
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not Provided");

    PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&database_url)
        .unwrap()
});

pub struct Postgres;
impl Postgres {
    pub fn pool() -> &'static sqlx::Pool<sqlx::Postgres> {
        unsafe { &*POOL }
    }
}

///
///
///
///
///
///

pub static REDIS_CLIENT: LazyLock<redis::Client> = LazyLock::new(|| {
    #[cfg(feature = "dotenv")]
    dotenv::dotenv().ok();

    let redis_url = std::env::var("REDIS_URL").expect("DATABASE_URL is not Provided");

    redis::Client::open(redis_url).unwrap()
});

pub struct Redis;
impl Redis {
    pub fn get_conn() -> crate::prelude::Result<redis::Connection> {
        REDIS_CLIENT.get_connection().map_err(|e| {
            tracing::debug!("Error when get Redis's Connection: {:#?}", e);
            crate::error::Error::Status(532)
        })
    }
}
