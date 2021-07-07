use std::fmt;

use rusqlite::Connection;
use structopt::StructOpt;

use error::GameError;
use session::Session;

mod error;
pub mod game;
pub mod session;

#[derive(StructOpt, Debug)]
#[structopt(name = "sidestacker")]
pub enum SideStacker {
    /// Create a new SideStacker Session
    Create(Params),
    /// Connect to a SideStacker Session
    Connect(Params),
}

#[derive(StructOpt, Debug)]
#[structopt(about = "SideStacker parameters")]
pub struct Params {
    #[structopt(short, long, default_value = "0.0.0.0")]
    address: String,
    #[structopt(short, long, default_value = "8080")]
    port: u32,
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
    let connection = Connection::open_in_memory()?;

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
    // let session = match SideStacker::from_args() {
    //     SideStacker::Create(params) => Session::new(params),
    //     SideStacker::Connect(params) => Session::connect(params),
    // };

    Session::try_new()
}
