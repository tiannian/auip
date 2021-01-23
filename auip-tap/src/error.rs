#[derive(Debug)]
pub enum Error {
    StdIOError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::StdIOError(e)
    }
}

pub type Result<R> = core::result::Result<R, Error>;
