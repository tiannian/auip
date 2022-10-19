#[derive(Debug)]
pub enum Error {
}

pub type Result<R> = core::result::Result<R, Error>;
