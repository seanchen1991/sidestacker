use std::convert::TryFrom;
use std::fmt;

use crate::error::GameError;

pub mod board;

/// The possible variants of a single slot in a Board.
#[derive(Debug, Clone, Copy, PartialEq)]
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

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Side::Left => write!(f, "L"),
            Side::Right => write!(f, "R"),
        }
    }
}

/// The directions in which a 4-length sequence of Slots constitutes a win.
#[derive(Debug)]
pub enum Direction {
    North,
    NorthWest,
    NorthEast,
    South,
    SouthWest,
    SouthEast,
    East,
    West,
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

/// A Row of the Board.
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
    /// Get the length of the Row.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether the Row contains no Blank Slots.
    pub fn is_full(&self) -> bool {
        self.0.iter().all(|slot| *slot != Slot::Blank)
    }

    /// Get the Slot at the given column index in the Row.
    pub fn get(&self, col: usize) -> &Slot {
        &self.0[col]
    }
}
