use std::fmt;

use super::*;
use crate::error::GameError;

/// Represents the game board.
#[derive(Debug)]
pub struct Board {
    pub rows: Vec<Row>,
    pub height: usize,
    pub width: usize,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (row_num, row) in self.rows.iter().enumerate() {
            writeln!(f, "{} {}", row_num, row)?;
        }

        Ok(())
    }
}

impl Board {
    /// Initializes a new Board with the specified height and width.
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            rows: (0..height)
                .map(|_| Row((0..width).map(|_| Slot::Blank).collect::<Vec<_>>()))
                .collect::<Vec<_>>(),
            height,
            width,
        }
    }

    /// Try to fetch a reference to a specified Row.
    pub fn try_get_row(&self, row_index: usize) -> Result<&Row, GameError> {
        let row = if let Some(row) = self.rows.get(row_index) {
            row
        } else {
            return Err(GameError::NonexistentRow);
        };

        Ok(row)
    }

    /// Try to fetch a mutable reference to a specified Row.
    pub fn try_get_row_mut(&mut self, row_index: usize) -> Result<&mut Row, GameError> {
        let row = if let Some(row) = self.rows.get_mut(row_index) {
            row
        } else {
            return Err(GameError::NonexistentRow);
        };

        Ok(row)
    }

    /// Insert the given Slot into the specified Row from the left.
    /// Returns the coordinates of the spot that becomes occupied.
    pub fn insert_from_left(
        &mut self,
        row_num: usize,
        slot: Slot,
    ) -> Result<(usize, usize), GameError> {
        let row = self.try_get_row_mut(row_num)?;

        if row.is_full() {
            return Err(GameError::FullRow);
        }

        for (col, spot) in row.0.iter_mut().rev().enumerate() {
            match spot {
                Slot::Blank => {
                    *spot = slot;
                    return Ok((row_num, row.len() - col - 1));
                }
                _ => continue,
            }
        }

        Err(GameError::FullRow)
    }

    /// Insert the given Slot into the specified Row from the right.
    /// Returns the coordinates of the spot that becomes occupied.
    pub fn insert_from_right(
        &mut self,
        row_num: usize,
        slot: Slot,
    ) -> Result<(usize, usize), GameError> {
        let row = self.try_get_row_mut(row_num)?;

        if row.is_full() {
            return Err(GameError::FullRow);
        }

        for (col, spot) in row.0.iter_mut().enumerate() {
            match spot {
                Slot::Blank => {
                    *spot = slot;
                    return Ok((row_num, col));
                }
                _ => continue,
            }
        }

        Err(GameError::FullRow)
    }

    /// Computes whether the game is finished or not, starting at the given row and column index.
    pub fn is_game_over(
        &self,
        row_num: usize,
        col: usize,
        slot: &Slot,
    ) -> Result<Option<Slot>, GameError> {
        if let Slot::Blank = slot {
            panic!("Found a Blank Slot where there should not have been one.");
        }

        // traverse the board in all 8 directions
        let search_results = vec![
            self.recurse(slot, row_num, col, 1, Direction::North)
                + self.recurse(slot, row_num, col, 1, Direction::South)
                - 1,
            self.recurse(slot, row_num, col, 1, Direction::East)
                + self.recurse(slot, row_num, col, 1, Direction::West)
                - 1,
            self.recurse(slot, row_num, col, 1, Direction::NorthEast)
                + self.recurse(slot, row_num, col, 1, Direction::SouthWest)
                - 1,
            self.recurse(slot, row_num, col, 1, Direction::NorthWest)
                + self.recurse(slot, row_num, col, 1, Direction::SouthEast)
                - 1,
        ];

        Ok(if search_results.iter().any(|result| *result == 4) {
            Some(*slot)
        } else {
            None
        })
    }

    /// Recursive helper function for traversing the Board.
    fn recurse(
        &self,
        slot: &Slot,
        row_num: usize,
        col: usize,
        len_so_far: u32,
        direction: Direction,
    ) -> u32 {
        // base case
        if let Slot::Blank = slot {
            return len_so_far;
        }

        match direction {
            Direction::North => match self.try_get_row(row_num.overflowing_sub(1).0) {
                Ok(row) => {
                    if slot == row.get(col) {
                        self.recurse(slot, row_num - 1, col, len_so_far + 1, direction)
                    } else {
                        len_so_far
                    }
                }
                Err(_) => len_so_far,
            },
            Direction::South => match self.try_get_row(row_num + 1) {
                Ok(row) => {
                    if slot == row.get(col) {
                        self.recurse(slot, row_num + 1, col, len_so_far + 1, direction)
                    } else {
                        len_so_far
                    }
                }
                Err(_) => len_so_far,
            },
            Direction::East => {
                let row = self.try_get_row(row_num).unwrap();

                if col < self.width - 1 {
                    if slot == row.get(col + 1) {
                        return self.recurse(slot, row_num, col + 1, len_so_far + 1, direction);
                    }
                }

                len_so_far
            }
            Direction::West => {
                let row = self.try_get_row(row_num).unwrap();

                if col > 0 {
                    if slot == row.get(col - 1) {
                        return self.recurse(slot, row_num, col - 1, len_so_far + 1, direction);
                    }
                }

                len_so_far
            }
            Direction::NorthEast => match self.try_get_row(row_num.overflowing_sub(1).0) {
                Ok(row) => {
                    if col < self.width - 1 {
                        if slot == row.get(col + 1) {
                            return self.recurse(
                                slot,
                                row_num - 1,
                                col + 1,
                                len_so_far + 1,
                                direction,
                            );
                        }
                    }

                    len_so_far
                }
                Err(_) => len_so_far,
            },
            Direction::NorthWest => match self.try_get_row(row_num.overflowing_sub(1).0) {
                Ok(row) => {
                    if col > 0 {
                        if slot == row.get(col - 1) {
                            return self.recurse(
                                slot,
                                row_num - 1,
                                col - 1,
                                len_so_far + 1,
                                direction,
                            );
                        }
                    }

                    len_so_far
                }
                Err(_) => len_so_far,
            },
            Direction::SouthEast => match self.try_get_row(row_num + 1) {
                Ok(row) => {
                    if col < self.width - 1 {
                        if slot == row.get(col + 1) {
                            return self.recurse(
                                slot,
                                row_num + 1,
                                col + 1,
                                len_so_far + 1,
                                direction,
                            );
                        }
                    }

                    len_so_far
                }
                Err(_) => len_so_far,
            },
            Direction::SouthWest => match self.try_get_row(row_num + 1) {
                Ok(row) => {
                    if col > 0 {
                        if slot == row.get(col - 1) {
                            return self.recurse(
                                slot,
                                row_num + 1,
                                col - 1,
                                len_so_far + 1,
                                direction,
                            );
                        }
                    }

                    len_so_far
                }
                Err(_) => len_so_far,
            },
        }
    }
}
