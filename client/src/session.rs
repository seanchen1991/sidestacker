use futures::{sink::SinkExt, StreamExt};
use std::convert::TryFrom;
use std::io::{self, prelude::*};

use crate::{
    error::ClientError,
    game::{board::Board, Slot},
    Connection, Move, Player, Response, Side, Turn,
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

/// The client's view of the game.
pub struct Session {
    /// The Board that the game is played on.
    pub board: Board,
    /// The Player on this client.
    pub player: Player,
    /// The Player whose turn it currently is.
    pub current_player: Player,
    /// The turns that have occurred over the course of the game.
    pub turns: Vec<Turn>,
}

impl Session {
    /// Initialize a new Session with a Board of the specified dimensions.
    pub fn new(player: Player, height: usize, width: usize) -> Self {
        Session {
            board: Board::new(height, width),
            turns: Vec::new(),
            player,
            current_player: Player::First,
        }
    }

    /// Run the game loop.
    pub async fn play(&mut self, connection: &mut Connection) -> Result<(), ClientError> {
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
                .map_err(|e| ClientError::InputError { source: e })?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| ClientError::InputError { source: e })?;

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

            let turn = Turn {
                source: self.player,
                mov,
            };
            connection
                .lines
                .send(&serde_json::to_string(&turn)?)
                .await?;

            loop {
                match connection.lines.next().await {
                    Some(Ok(ref resp)) => {
                        let response: Response = serde_json::from_str(&resp)?;

                        if let Response::Acknowledged = response {
                            break;
                        }
                    }
                    _ => {}
                }
            }

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

            self.turns.push(turn);

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
