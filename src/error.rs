use std::{self, fmt, io};

/// A network or validation error
#[derive(Debug)]
pub enum Error {
    JSONWebToken(jsonwebtoken::errors::Error),
    ConnectionError(Box<dyn std::error::Error + Send + Sync + 'static>),
    InvalidKey,
    InvalidToken,
    InvalidIssuer,
    InvalidAudience,
    InvalidHostedDomain,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ConnectionError(ref err) => err.fmt(f),
            Error::JSONWebToken(ref err) => err.fmt(f),
            Error::InvalidKey => f.write_str("Token does not match any known key"),
            Error::InvalidToken => f.write_str("Token was not recognized by google"),
            Error::InvalidIssuer => f.write_str("Token was not issued by google"),
            Error::InvalidAudience => f.write_str("Token is for a different google application"),
            Error::InvalidHostedDomain => {
                f.write_str("User is not a member of the hosted domain(s)")
            }
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::ConnectionError(Box::new(err))
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(err: jsonwebtoken::errors::Error) -> Error {
        Error::JSONWebToken(err)
    }
}
