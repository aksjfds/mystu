impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoCare => f.write_str("Error::NoCare"),
            Error::Status(code) => f.write_fmt(format_args!("Error::Status({})", code)),
        }
    }
}

// 不想处理的，返回一个404
// 要处理的，一般都需要抛到上一层，响应给客户端发生什么了
#[derive(Debug)]
pub enum Error {
    NoCare,
    Status(u16),
}
