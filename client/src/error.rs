use std::error::Error;
use std::fmt;
use std::io;

use serde_json::Error as JsonError;
use tokio_util::codec;

/// The error types that may arise while the game is running.
#[derive(Debug)]
pub enum ClientError {
    /// Attempted to insert into a full row.
    FullRow,
    /// Attempted to fetch a non-existent row.
    NonexistentRow,
    /// A player specified a move in an invalid format.
    InvalidMoveFormat,
    /// A player specified a side that is not valid.
    InvalidSide,
    /// Can't join a game because it is already at capacity.
    GameFull,
    /// There was an error reading or writing input.
    InputError { source: io::Error },
    /// An error occurred with the game server.
    ServerError(String),
    /// There was a connection error.
    ConnectionError(String),
    /// An error occurred while serializing or deserializing.
    SerializationError { source: JsonError },
    /// An error occurred while encoding or decoding a line.
    CodecError { source: codec::LinesCodecError },
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientError::FullRow => write!(f, "Row is full. Please pick a different one."),
            ClientError::NonexistentRow => write!(f, "That row doesn't exist. Please pick a different one."),
            ClientError::InputError { source } => write!(f, "There was an error reading/writing input: {}", source),
            ClientError::InvalidMoveFormat => write!(f, "Please specify your move with a number indicating the row and a letter indicating the side ('l' or 'r'), with no spaces in between them."),
            ClientError::InvalidSide => write!(f, "Please specify a side with a letter, 'l' or 'r'."),
            ClientError::ServerError(s) => write!(f, "An error occurred with the game server: {}", s),
            ClientError::ConnectionError(s) => write!(f, "There was a connection error: {}", s),
            ClientError::SerializationError { source } => write!(
                f,
                "An error occurred while serializing or deserializing: {}",
                source
            ),
            ClientError::CodecError { source } => write!(
                f,
                "An error occurred while encoding or decoding a line: {}",
                source
            ),
            ClientError::GameFull => write!(
                f,
                "Game is at max capacity and can't accept any more players 😞"
            ),
        }
    }
}

impl From<io::Error> for ClientError {
    fn from(source: io::Error) -> Self {
        Self::InputError { source }
    }
}

impl From<codec::LinesCodecError> for ClientError {
    fn from(source: codec::LinesCodecError) -> Self {
        Self::CodecError { source }
    }
}

impl From<JsonError> for ClientError {
    fn from(source: JsonError) -> Self {
        Self::SerializationError { source }
    }
}

impl Error for ClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InputError { source } => Some(source),
            Self::CodecError { source } => Some(source),
            Self::SerializationError { source } => Some(source),
            _ => None,
        }
    }
}
