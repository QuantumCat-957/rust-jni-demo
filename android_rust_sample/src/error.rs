#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("unauthorized")]
    Unauthorized,
    #[error("transport has been closed")]
    TransportHasBeenClosed,
    #[error("transport disconnected")]
    TransportDisconnected(Vec<u8>),
    #[error("invalid packet")]
    InvalidPacket,
    #[error("io error: `{0}`")]
    IO(String),
    #[error("crypto error: `{0}`")]
    Crypto(String),
    #[error("Http error: `{0}`")]
    Http(String),
    #[error("Jni wrapper error: `{0}`")]
    JniWrapper(String),
}

impl Error {
    pub fn get_status_code(&self) -> i32 {
        match self {
            Error::Unauthorized => 401,
            Error::TransportHasBeenClosed => 2001,
            Error::TransportDisconnected(_) => 2002,
            Error::InvalidPacket => 2003,
            Error::IO(_) => 2004,
            Error::Crypto(_) => 2005,

            Error::Http(_) => 2008,
            Error::JniWrapper(_) => 2009,
            // SystemError::Net(_) => 6300,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value.to_string())
    }
}

impl From<jni::errors::Error> for Error {
    fn from(value: jni::errors::Error) -> Self {
        Self::IO(value.to_string())
    }
}
