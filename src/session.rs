use std::convert::TryFrom;
use std::io::{self, prelude::*};

use crate::{
    board::{Side, Slot, Move, Board},
    error::GameError,
    Player,
};

static WELCOME: &str = "Welcome to SideStacker!
On your turn, specify your move with the format 
`[ROW NUMBER][SIDE]` with no spaces in between.

The following are examples of valid moves:
2R, 5r, 1l, 3L.

The game ends when there are no spaces left 
available, or when a player has four consecutive
pieces on a diagonal, column, or row.
";

/// The state of a single game.
pub struct Session {
    pub board: Board,
    pub game_over: bool,
    pub current_player: Player,
}

impl Session {
    pub fn new() -> Self {
        Session {
            board: Board::new(),
            game_over: false,
            current_player: Player::First,
        }
    }

    pub fn run(&mut self) -> Result<(), GameError> {
        println!("{}", WELCOME);

        while !self.game_over {
            println!("{}", self.board);
            println!("{} player's turn:", self.current_player);
            println!("What's the move?");

            io::stdout()
                .flush()
                .map_err(|e| GameError::InputError { source: e })?;

            let mut command = String::new(); 
            io::stdin()
                .read_line(&mut command)
                .map_err(|e| GameError::InputError { source: e })?;

            // parse the command into a Move type 
            let mov = match Move::try_from(command) {
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
            match mov.side {
                Side::Left => {
                    if let Err(e) = self.board.insert_from_left(mov.row, slot) {
                        println!("{}", e);
                        continue;
                    }
                },
                Side::Right => {
                    if let Err(e) = self.board.insert_from_right(mov.row, slot) {
                        println!("{}", e);
                        continue;
                    }
                },
            }

            // check if the game is over
            match self.board.is_game_over() {
                Ok(slot) => {
                    match slot {
                        Slot::Blank => 
                        Slot::X => {
                            println!("Game won by First Player!");
                            break;
                        },
                        Slot::O => {
                            println!("Game won by Second Player!");
                            break;
                        },
                    }
                }
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            }

            // if not, pass the turn over to the other player  

        }

        Ok(())
    }
}