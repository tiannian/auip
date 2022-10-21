#![no_std]

#[cfg(test)]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

mod error;
pub use error::*;

mod device;
pub use device::*;

pub mod interface;
pub use interface::Interface;

pub mod storage;

pub mod utils;
