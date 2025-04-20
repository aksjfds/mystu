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
    #[tracing::instrument(name = "signup", level = "debug", skip(self, req))]
    async fn async_handle(self, req: Request) -> Result<Response> {
        Ok(Response::new().status(404))
    }
}
