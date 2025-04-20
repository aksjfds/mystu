use crate::prelude::*;
use glacier::prelude::*;

///
/// 
/// 
/// 
/// 
/// 

pub struct SignUp;

impl HandleReq<Error> for SignUp {
    async fn async_handle(self, _req: Request) -> Result<Response> {
        Ok(Response::new().status(404))
    }
}
