use std::error::Error;
use std::fmt;
use std::io;

use serde_json::Error as JsonError;
use tokio_util::codec;

#[derive(Debug)]
pub enum ServerError {
    /// Game is already full; can't connect more Players.
    GameFull,
    /// An I/O occurred.
    IoError { source: io::Error },
    /// An error occurred while encoding or decoding a line.
    CodecError { source: codec::LinesCodecError },
    /// A Player attempted to take a turn when it isn't their turn.
    NotYourTurn,
    /// An error occurred while serializing or deserializing.
    SerializationError { source: JsonError },
    /// An error occurred with the database.
    DatabaseError { source: rusqlite::Error },
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerError::GameFull => write!(
                f,
                "Game is at max capacity and can't accept any more players 😞"
            ),
            ServerError::IoError { source } => write!(f, "An I/O error occurred: {}", source),
            ServerError::CodecError { source } => write!(
                f,
                "An error occurred while encoding or decoding a line: {}",
                source
            ),
            ServerError::NotYourTurn => write!(f, "It isn't your turn!"),
            ServerError::SerializationError { source } => write!(
                f,
                "An error occurred while serializing or deserializing: {}",
                source
            ),
            ServerError::DatabaseError { source } => {
                write!(f, "An error occurred with the database: {}", source)
            }
        }
    }
}

impl From<io::Error> for ServerError {
    fn from(source: io::Error) -> Self {
        Self::IoError { source }
    }
}

impl From<codec::LinesCodecError> for ServerError {
    fn from(source: codec::LinesCodecError) -> Self {
        Self::CodecError { source }
    }
}

impl From<JsonError> for ServerError {
    fn from(source: JsonError) -> Self {
        Self::SerializationError { source }
    }
}

impl From<rusqlite::Error> for ServerError {
    fn from(source: rusqlite::Error) -> Self {
        Self::DatabaseError { source }
    }
}

impl Error for ServerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IoError { source } => Some(source),
            Self::CodecError { source } => Some(source),
            Self::SerializationError { source } => Some(source),
            Self::DatabaseError { source } => Some(source),
            _ => None,
        }
    }
}
