use std::sync::LazyLock;

pub static mut POOL: LazyLock<sqlx::Pool<sqlx::Postgres>> = LazyLock::new(|| todo!());

pub struct Postgres;
impl Postgres {
    pub fn init() {
        use sqlx::postgres::PgPoolOptions;

        dotenv::dotenv().ok();
        
        unsafe {
            POOL = LazyLock::new(|| {
                let database_url =
                    std::env::var("DATABASE_URL").expect("DATABASE_URL is not Provided");

                PgPoolOptions::new()
                    .max_connections(5)
                    .connect_lazy(&database_url)
                    .unwrap()
            })
        }
    }
    pub fn pool() -> &'static sqlx::Pool<sqlx::Postgres> {
        unsafe { &*POOL }
    }
}
