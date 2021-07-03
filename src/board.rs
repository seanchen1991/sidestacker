use std::fmt;
use std::convert::TryFrom;

use crate::error::GameError;

/// The possible variants of a single slot in a Board.
#[derive(Debug)]
pub enum Slot {
    /// A blank slot owned by neither player.
    Blank,
    /// A slot owned by the player playing X.
    X,
    /// A slot owned by the player playing O.
    O,
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Slot::Blank => write!(f, "_"),
            Slot::X => write!(f, "X"),
            Slot::O => write!(f, "O"),
        }
    }
}

/// The sides from which Players may choose to insert a slot.
#[derive(Debug)]
pub enum Side {
    Left,
    Right,
}

/// Represents a Player's move.
#[derive(Debug)]
pub struct Move {
    pub side: Side,
    pub row: usize,
}

impl TryFrom<String> for Move {
    type Error = GameError;

    fn try_from(command: String) -> Result<Self, Self::Error> {
        let chars = command.chars().collect::<Vec<_>>();

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

#[derive(Debug)]
pub struct Row(Vec<Slot>);

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ ")?;
        
        for slot in self.0.iter() {
            write!(f, "{} ", slot.to_string())?;
        }
        
        write!(f, "]")
    }
}

impl Row {
    pub fn is_full(&self) -> bool {
        self.0.iter().any(|slot| if let Slot::Blank = slot { false } else { true })
    }
}

/// Represents the game board.
#[derive(Debug)]
pub struct Board(Vec<Row>);

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (row_num, row) in self.0.iter().enumerate() {
            writeln!(f, "{} {}", row_num + 1, row)?;
        }
        
        Ok(())
    }
}

impl Board {
    /// Initializes a new 7x7 Board.
    // TODO: Make this so that the size of the Board can be varied.
    pub fn new() -> Self {
        Self (
            (0..7)
                .map(|_| {
                    Row (
                        (0..7)
                            .map(|_| Slot::Blank)
                            .collect::<Vec<_>>()
                    )
                })
                .collect::<Vec<_>>()
            
        )
    }
    
    /// Try to fetch the given Row.
    pub fn try_get_row(&mut self, row_index: usize) -> Result<&mut Row, GameError> {
        let row = if let Some(row) = self.0.get_mut(row_index) {
            row
        } else {
            return Err(GameError::NonexistentRow);
        };

        Ok(row)
    }

    /// Insert the given Slot into the specified Row from the left.
    pub fn insert_from_left(&mut self, row_num: usize, slot: Slot) -> Result<(), GameError> {
        let row = self.try_get_row(row_num - 1)?;

        if row.is_full() {
            return Err(GameError::FullRow);
        }

        for spot in row.0.iter_mut() {
            match spot {
                Slot::Blank => {
                    *spot = slot;
                    break;
                },
                _ => continue,
            }
        }

        Ok(())
    }

    /// Insert the given Slot into the specified Row from the left.
    pub fn insert_from_right(&mut self, row_num: usize, slot: Slot) -> Result<(), GameError> {
       let row = self.try_get_row(row_num - 1)?; 

        if row.is_full() {
            return Err(GameError::FullRow);
        }

        for spot in row.0.iter_mut().rev() {
            match spot {
                Slot::Blank => {
                    *spot = slot;
                    break;
                },
                _ => continue,
            }
        }

        Ok(())
    }

    pub fn is_game_over(&self) -> Result<Slot, GameError> {
        
    }
}