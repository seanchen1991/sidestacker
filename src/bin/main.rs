use std::process;

use sidestacker::config::get_config;

fn main() {
    let config = get_config().expect("Error: Failed to read configuration");

    // Creator of the game calls `init` to create a new game session
    // on a port.
    // Second player connects to the game session on the same port.
    let mut game = match sidestacker::init() {
        Ok(session) => session,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = game.run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    println!("{}", game.board);
}
