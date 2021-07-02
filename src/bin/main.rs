use std::process;

fn main() {
    // Creator of the game calls `init` to create a new game session
    // on a port.
    // Second player connects to the game session on the same port.
    let game = sidestacker::init().unwrap();

    println!("{}", game.board);
}
