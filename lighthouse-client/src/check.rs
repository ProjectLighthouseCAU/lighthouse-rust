use lighthouse_protocol::ServerMessage;

use crate::{Error, Result};

pub trait Check: Sized {
    fn check(self) -> Result<Self>;
}

impl<P> Check for ServerMessage<P> {
    fn check(self) -> Result<Self> {
        if self.code >= 200 && self.code < 300 {
            Ok(self)
        } else {
            Err(Error::Server { code: self.code, message: self.response, warnings: self.warnings })
        }
    }
}
