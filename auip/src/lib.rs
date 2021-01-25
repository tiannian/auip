#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![no_std]

pub mod interface;
pub mod phy;

#[derive(Debug)]
pub enum Error {}

pub type Result<R> = core::result::Result<R, Error>;
