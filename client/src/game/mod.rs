use std::fmt;

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
