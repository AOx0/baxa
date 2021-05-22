/// Error. Used for error handling.
/// 
/// It wraps a String containing the error.
pub struct Error(String);

/// Display trait for Error
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

/// Debug trait for Error
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

/// Convert std::io::Error to Error 
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error(format!("{}", error))
    }
}

/// Convert std::string::FromUtf8Error to Error 
impl std::convert::From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error(format!("{}", error))
    }
}

impl std::convert::From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Error(format!("{}", error))
    }
}

impl Error {
    pub fn from(msg: &str) -> Self {
        Error(msg.to_string())
    }
}