use lighthouse_protocol::ServerMessage;

use crate::{Error, Result};

pub trait Check: Sized {
    fn check(self) -> Result<Self>;
}

impl Check for ServerMessage {
    fn check(self) -> Result<Self> {
        if self.code == 200 {
            Ok(self)
        } else {
            Err(Error::Server { code: self.code, message: self.response, warnings: self.warnings })
        }
    }
}
