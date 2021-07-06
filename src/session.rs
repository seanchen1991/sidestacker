use std::convert::TryFrom;
use std::io::{self, prelude::*};

use crate::{
    error::GameError,
    game::{board::Board, Move, Side, Slot},
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

/// The state of a single game.
pub struct Session {
    pub board: Board,
    pub current_player: Player,
}

impl Session {
    pub fn new() -> Self {
        Session {
            board: Board::new(7, 7),
            current_player: Player::First,
        }
    }

    pub fn run(&mut self) -> Result<(), GameError> {
        println!("{}", WELCOME);

        loop {
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

            dbg!(&mov);

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
