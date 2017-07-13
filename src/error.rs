use hyper;
use oauth;
use serde_json as json;
use std::error;
use std::fmt;
use std::io;
use xml;

/// An error encountered when communicating with the Xero API.
#[derive(Debug)]
pub enum Error {
    /// An error reported by Xero.
    Xero(RequestError),
    /// A networking error communicating with the Xero server.
    Http(hyper::Error),
    /// An error reading the response body.
    Io(io::Error),
    /// An error while authenticating with OAuth
    OAuth(oauth::Error),
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
            Error::OAuth(ref err) => write!(f, ": {}", err),
            Error::Conversion(ref err) => write!(f, ": {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Xero(_) => "error reported by xero",
            Error::Http(_) => "error communicating with xero",
            Error::Io(_) => "error reading response from xero",
            Error::OAuth(_) => "error performing oauth with xero",
            Error::Conversion(_) => "error converting between wire format and Rust types",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Xero(ref err) => Some(err),
            Error::Http(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::OAuth(ref err) => Some(err),
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

impl From<oauth::Error> for Error {
    fn from(err: oauth::Error) -> Error {
        Error::OAuth(err)
    }
}

impl From<xml::writer::Error> for Error {
    fn from(err: xml::writer::Error) -> Error {
        Error::Conversion(Box::new(err))
    }
}

impl From<json::Error> for Error {
    fn from(err: json::Error) -> Error {
        Error::Conversion(Box::new(err))
    }
}

/// An error reported by Xero in a request's response.
#[derive(Debug,Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ErrorMessage {
    message: String,
}
#[derive(Debug,Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ErrorElement {
    has_validation_errors: bool,
    validation_errors: Vec<ErrorMessage>,
    // warnings: Vec<_>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RequestError {
    Status(StatusError),
    Validation(ValidationError),
    UnknownError,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatusError {
    id: String,
    status: String,
    #[serde(default)]
    provider_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ValidationError {
    error_number: i64,
    #[serde(rename = "Type")]
    error_type: String,
    #[serde(default)]
    message: String,
    #[serde(default)]
    elements: Vec<ErrorElement>,
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
