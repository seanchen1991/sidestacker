use std::convert::TryFrom;
use std::fmt;
use std::net::{SocketAddr, TcpStream};

use serde::{Serialize, Deserialize};
use structopt::StructOpt;

use error::GameError;
use session::Session;

mod error;
pub mod game;
pub mod session;

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
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Player {
    /// First Player
    First,
    /// Second Player
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

/// The sides from which Players may choose to insert a slot.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Side {
    Left,
    Right,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Side::Left => write!(f, "L"),
            Side::Right => write!(f, "R"),
        }
    }
}

/// Represents a Player's move.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Move {
    pub side: Side,
    pub row: usize,
}

impl TryFrom<String> for Move {
    type Error = GameError;

    fn try_from(command: String) -> Result<Self, Self::Error> {
        let chars = command.trim().chars().collect::<Vec<_>>();

        if chars.len() != 2 {
            return Err(GameError::InvalidMoveFormat);
        }

        let row = match chars[0].to_digit(10) {
            Some(num) => num as usize,
            None => return Err(GameError::NonexistentRow),
        };

        let side = match chars[1] {
            'l' | 'L' => Side::Left,
            'r' | 'R' => Side::Right,
            _ => return Err(GameError::InvalidSide),
        };

        Ok(Self { row, side })
    }
} 

impl fmt::Display for Move {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}{})", self.row, self.side)
   }
}

/// A Player's turn.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Turn {
    source: Player,
    mov: Move,
}

/// Requests the server receives from clients.
#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    /// A client requests to join the game.
    Join,
    /// A client submits a `Turn` action.
    Turn(Turn),
}

/// The server's responses to client requests.
#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    /// There is enough capacity in the game. Tell the client which
    /// Player they are.
    Welcome { player: Player },
    /// There are enough Players for the game to start.
    GameStart,
    /// There is not enough capacity in the game.
    GameFull,
    /// A Player attempted to act out of turn.
    NotYourTurn,
    /// Server sends the current Player's Turn to the other Player.
    Turn(Turn),
    /// Server acknowledges a Player's proposed Turn.
    Acknowledged,
    /// The other Player disconnected.
    PlayerDisconnected,
    /// An internal server error occurred.
    ServerError,
}

/// Grabs CLI args and either creates a new game or connects to a pre-existing one.
pub fn init() -> Result<Session, GameError> {
    let SideStacker::Connect(params) = SideStacker::from_args(); 

    let stream = match TcpStream::connect(params.address) {
        Ok(stream) => stream,
        Err(_) => return Err(GameError::ServerError),
    };

    // receive the server's response
    // if it is a `GameReady` response of `Player 2` response, then 
    // create the Session

    Ok(Session::new(stream))
}
