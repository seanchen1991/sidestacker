use std::convert::TryFrom;
use std::fmt;
use std::io::{self, prelude::*};
use std::net::TcpStream;

use crate::{
    error::GameError,
    game::{board::Board, Move, Side, Slot},
    init_db,
    Player,
};

static WELCOME: &str = "Welcome to SideStacker!
On your turn, specify your move with the format 
`[ROW-NUMBER][SIDE]` with no spaces in between.

The following are examples of valid moves:
2R, 5r, 1l, 3L.

The game ends when there are no spaces left 
available, or when a player has four consecutive
pieces on a diagonal, column, or row.
";

/// Stores the Moves made by both Players in order.
#[derive(Debug)]
pub struct Turns(Vec<Move>);

impl fmt::Display for Turns {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for mov in self.0.iter() {
            write!(f, "{}\n", mov)?;
        }

        Ok(())
    }
}

impl Turns {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

/// The state of a single game.
pub struct Session {
    /// The Board that the game is played on.
    pub board: Board,
    /// The current Player.
    pub current_player: Player,
    /// The turns that have occurred over the course of the game.
    pub turns: Turns,
    /// The stream for communicating with the server.
    pub stream: TcpStream,
}

impl Session {
    /// Initialize a new Session with a 7x7 Board.
    pub fn new(stream: TcpStream) -> Self {
        Session {
            board: Board::new(7, 7),
            current_player: Player::First,
            turns: Turns(Vec::new()),
            stream,
        }
    }

    /// Persists the turns of the game to the database.
    // pub fn save_game(&self) -> Result<(), GameError> {
    //     self.database_connection
    //         .execute(
    //             "INSERT INTO games (turns) values (?1)",
    //             &[&self.turns.to_string()]
    //         )?;

    //     Ok(())
    // }

    /// Start the game if all Players are ready, or wait for more
    /// Players to connect.
    pub fn run(&mut self) -> Result<(), GameError> {
        let mut buffer = vec![0 as u8; 256];

        loop {
            while let Ok(bytes_read) = self.stream.read(&mut buffer) {
                if bytes_read == 0 {
                    return Err(GameError::ConnectionError("Received no response from server.".to_string()));
                }

                
            }
        }
    }

    /// Execute a single turn of the game.
    pub fn play(&mut self) -> Result<(), GameError> {
        println!("{}", WELCOME);

        loop {
            // Check if the game has resulted in a tie
            if self.turns.len() == self.board.height * self.board.width {
                println!("Game ended in a tie!");
                break;
            }

            println!("{}", self.board);
            println!("{} player's turn:", self.current_player);
            println!("What's the move?");

            io::stdout()
                .flush()
                .map_err(|e| GameError::InputError { source: e })?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| GameError::InputError { source: e })?;

            if input.trim().to_lowercase() == "quit" {
                break;
            }

            // parse the input into a Move
            let mov = match Move::try_from(input) {
                Ok(mov) => mov,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };

            let slot = match self.current_player {
                Player::First => Slot::X,
                Player::Second => Slot::O,
            };

            // update the Board state
            let (row, col) = match mov.side {
                Side::Left => match self.board.insert_from_left(mov.row, slot) {
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                    Ok((row, col)) => (row, col),
                },
                Side::Right => match self.board.insert_from_right(mov.row, slot) {
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                    Ok((row, col)) => (row, col),
                },
            };

            // log the turn 
            self.turns.0.push(mov);

            // check if the game is over
            match self.board.is_game_over(row, col, &slot) {
                Ok(slot) => match slot {
                    Some(Slot::X) => {
                        println!("Game won by First Player!");
                        break;
                    }
                    Some(Slot::O) => {
                        println!("Game won by Second Player!");
                        break;
                    }
                    Some(Slot::Blank) => {
                        panic!("Returned a blank Slot where it should not have been returned.")
                    }
                    None => {
                        self.current_player = !self.current_player;
                        continue;
                    }
                },
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            }
        }

        Ok(())
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        println!("Saving game to database...");

        self.save_game().expect("Error: Failed to persist game.");
    }
}