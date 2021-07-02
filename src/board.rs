use std::fmt;

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
}