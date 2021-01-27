#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(min_const_generics)]
#![no_std]

pub mod interface;
pub mod phy;
pub mod stack;
pub mod auip;

#[derive(Debug)]
pub enum Error {}

pub type Result<R> = core::result::Result<R, Error>;
