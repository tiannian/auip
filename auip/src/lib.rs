#![no_std]

#[cfg(test)]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

mod error;
pub use error::*;

mod device;
pub use device::*;

mod interface;
pub use interface::*;

pub mod storage;

pub mod utils;

pub mod consts;
