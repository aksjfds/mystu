use crate::prelude::*;
use glacier::prelude::*;

pub struct Get;
impl HandleReq<Error> for Get {
    fn filter(self, req: Request) -> Result<Request> {
        match req.method() == "GET" {
            true => Ok(req),
            false => Err(Error::Status(404)),
        }
    }
}

pub struct Post;
impl HandleReq<Error> for Post {
    fn filter(self, req: Request) -> Result<Request> {
        match req.method() == "POST" {
            true => Ok(req),
            false => Err(Error::Status(404)),
        }
    }
}
