mod check;
mod connect;
mod constants;
mod error;
mod lighthouse;
mod spawn;

pub use check::*;
pub use connect::*;
pub use constants::*;
pub use error::*;
pub use lighthouse::*;
pub use spawn::*;

pub use lighthouse_protocol as protocol;

/// Small convenience macro that expresses the root path.
#[macro_export]
macro_rules! root {
    () => {
        &[] as &[&str]
    };
}
