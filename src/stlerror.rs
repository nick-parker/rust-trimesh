use std::error;
use std::fmt;
use std::io;
use std::num;

#[derive(Debug)]
pub enum StlError{
	Io(io::Error),
	Parse(num::ParseFloatError),
	BadLine(String),
	BadStl(String),
}

impl fmt::Display for StlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            StlError::Io(ref err) => write!(f, "IO error: {}", err),
            StlError::Parse(ref err) => write!(f, "Parse error: {}", err),
            StlError::BadLine(ref s) => write!(f,
            	".stl file has invalid line start: {}", s),
            StlError::BadStl(ref s) => write!(f,"Bad .stl: {}", s),
        }
    }
}

impl error::Error for StlError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            StlError::Io(ref err) => err.description(),
            // Normally we can just write `err.description()`, but the error
            // type has a concrete method called `description`, which conflicts
            // with the trait method. For now, we must explicitly call
            // `description` through the `Error` trait.
            StlError::Parse(ref err) => error::Error::description(err),
            StlError::BadLine(ref s) => s,
            StlError::BadStl(ref s) => s,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types (either `&io::Error` or `&num::ParseIntError`)
            // to a trait object `&Error`. This works because both error types
            // implement `Error`.
            StlError::Io(ref err) => Some(err),
            StlError::Parse(ref err) => Some(err),
            StlError::BadLine(_) => None,
            StlError::BadStl(_) => None
        }
    }
}