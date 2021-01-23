use core::future::Future;

pub trait Device {
    fn receive(&mut self);
}