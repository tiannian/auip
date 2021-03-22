#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![no_std]

pub mod auip;
// pub mod interface;
pub mod phy;
pub mod stack;

#[derive(Debug)]
pub enum Error {
    DriverPacketError,
}

pub type Result<R> = core::result::Result<R, Error>;
