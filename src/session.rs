use crate::board::Board;

/// The state of a single game.
pub struct Session {
    pub board: Board,
}

impl Session {
    pub fn new() -> Self {
        Session {
            board: Board::new()
        }
    }
}