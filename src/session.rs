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
    /// The Board that the game is played on.
    pub board: Board,
    /// The current Player.
    pub current_player: Player,
}

impl Session {
    /// Initialize a new Session with a 7x7 Board.
    pub fn new() -> Self {
        Session {
            board: Board::new(7, 7),
            current_player: Player::First,
        }
    }

    /// Execute a single turn of the game.
    pub fn run(&mut self) -> Result<(), GameError> {
        println!("{}", WELCOME);

        loop {
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
