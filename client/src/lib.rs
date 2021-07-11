use std::fmt;
use std::net::{SocketAddr, TcpStream};

use rusqlite::Connection;
use structopt::StructOpt;

use error::GameError;
use session::Session;

mod error;
pub mod game;
pub mod session;

static DB_PATH: &str = "db/games.db";

#[derive(StructOpt, Debug)]
#[structopt(name = "sidestacker")]
pub enum SideStacker {
    /// Connect to a SideStacker Session
    Connect(Params),
}

#[derive(StructOpt, Debug)]
#[structopt(about = "SideStacker parameters")]
pub struct Params {
    #[structopt(short, long, default_value = "0.0.0.0:8080")]
    address: SocketAddr,
}

/// The Player variants.
#[derive(Debug, Clone, Copy)]
pub enum Player {
    /// Player 1
    First,
    /// Player 2
    Second,
}

impl std::ops::Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Player::First => Player::Second,
            Player::Second => Player::First,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::First => write!(f, "First"),
            Player::Second => write!(f, "Second"),
        }
    }
}

pub fn init_db() -> Result<Connection, GameError> {
    let connection = Connection::open(DB_PATH)?;

    connection.execute(
        "CREATE TABLE games (
            id INTEGER PRIMARY KEY,
            turns TEXT NOT NULL
        )",
        [],
    )?;

    Ok(connection)
}

/// Grabs CLI args and either creates a new game or connects to a pre-existing one.
pub fn init() -> Result<Session, GameError> {
    let SideStacker::Connect(params) = SideStacker::from_args(); 

    let stream = match TcpStream::connect(params.address) {
        Ok(stream) => stream,
        Err(e) => return Err(GameError::ConnectionError(e.to_string())),
    };

    Ok(Session::new(stream))
}
