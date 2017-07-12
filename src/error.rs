use hyper;
use serde_json as json;
use std::error;
use std::fmt;
use std::io;

/// An error encountered when communicating with the Xero API.
#[derive(Debug)]
pub enum Error {
    /// An error reported by Xero.
    Xero(RequestError),
    /// A networking error communicating with the Xero server.
    Http(hyper::Error),
    /// An error reading the response body.
    Io(io::Error),
    /// An error converting between wire format and Rust types.
    Conversion(Box<error::Error + Send>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(error::Error::description(self))?;
        match *self {
            Error::Xero(ref err) => write!(f, ": {}", err),
            Error::Http(ref err) => write!(f, ": {}", err),
            Error::Io(ref err) => write!(f, ": {}", err),
            Error::Conversion(ref err) => write!(f, ": {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Xero(_) => "error reported by xero maps",
            Error::Http(_) => "error communicating with xero maps",
            Error::Io(_) => "error reading response from xero maps",
            Error::Conversion(_) => "error converting between wire format and Rust types",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Xero(ref err) => Some(err),
            Error::Http(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::Conversion(ref err) => Some(&**err),
        }
    }
}

impl From<RequestError> for Error {
    fn from(err: RequestError) -> Error {
        Error::Xero(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Http(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<json::Error> for Error {
    fn from(err: json::Error) -> Error {
        Error::Conversion(Box::new(err))
    }
}

/// An error reported by Xero in a request's response.
#[derive(Debug, Default, Deserialize)]
pub struct RequestError {
     // TODO: What is in a RequestError?
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RequestError") // TODO: What is in a RequestError?
    }
}

impl error::Error for RequestError {
    fn description(&self) -> &str {
        "error reported by xero maps"
    }
}
