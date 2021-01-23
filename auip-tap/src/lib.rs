mod error;
pub use error::{Error, Result};

mod tap;
pub use tap::open_tap_device;
