impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoCare => f.write_str("Error::NoCare"),
            Error::Status(code) => f.write_fmt(format_args!("Error::Status({})", code)),
        }
    }
}

/**
# Error
   * 参数错误 - 404
   * JWT错误 - 401
   * PostgreSQL数据库错误 - 531
   * Redis数据库错误 - 532
   * 加密Token错误 - 533
   * 需要自定义的错误 - 7xx
*/
#[derive(Debug)]
pub enum Error {
    NoCare,
    Status(u16),
}

impl Default for Error {
    fn default() -> Self {
        Error::Status(404)
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        tracing::debug!("Error when use PostgreSQL: {:#?}", value);
        Error::Status(531)
    }
}

impl From<redis::RedisError> for Error {
    fn from(value: redis::RedisError) -> Self {
        tracing::debug!("Error when use Redis: {:#?}", value);
        Error::Status(532)
    }
}
