use crate::prelude::*;
use glacier::prelude::*;

pub struct NotFound;

impl HandleReq<Error> for NotFound {
    fn handle(self, _req: Request) -> Result<Response> {
        Err(Error::Status(404))
    }
}
