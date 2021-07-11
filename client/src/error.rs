use std::error::Error;
use std::fmt;
use std::io;

/// The error types that may arise while the game is running.
#[derive(Debug)]
pub enum GameError {
    /// Attempted to insert into a full row.
    FullRow,
    /// Attempted to fetch a non-existent row.
    NonexistentRow,
    /// A player specified a move in an invalid format.
    InvalidMoveFormat,
    /// A player specified a side that is not valid.
    InvalidSide,
    /// There was an error reading or writing input.
    InputError { source: io::Error },
    /// An error occurred with the database.
    DatabaseError { source: rusqlite::Error },
    /// An error occurred with the connection.
    ConnectionError(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::FullRow => write!(f, "Row is full. Please pick a different one."),
            GameError::NonexistentRow => write!(f, "That row doesn't exist. Please pick a different one."),
            GameError::InputError { source } => write!(f, "There was an error reading/writing input: {}", source),
            GameError::InvalidMoveFormat => write!(f, "Please specify your move with a number indicating the row and a letter indicating the side ('l' or 'r'), with no spaces in between them."),
            GameError::InvalidSide => write!(f, "Please specify a side with a letter, 'l' or 'r'."),
            GameError::DatabaseError { source } => write!(f, "An error occurred with the database: {}", source),
            GameError::ConnectionError(s) => write!(f, "A connection error occurred: {}", s),
        }
    }
}

impl From<io::Error> for GameError {
    fn from(source: io::Error) -> Self {
        Self::InputError { source }
    }
}

impl From<rusqlite::Error> for GameError {
    fn from(source: rusqlite::Error) -> Self {
        Self::DatabaseError { source }
    }
}

impl Error for GameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InputError { source } => Some(source),
            Self::DatabaseError { source } => Some(source),
            Self::ConnectionError { source } => Some(source),
            _ => None,
        }
    }
}
