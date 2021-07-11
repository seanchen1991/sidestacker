use std::process;

fn main() {
    // Server is started first separately
    // Client attempts to connect to the server using the specified address
    let mut session = match client::init() {
        Ok(session) => session,
            // If connection is successful, either start the game immediately, or 
            // wait for another player to connect
            // Game doesnt' start until the server sends a `Ready` response to clients 
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = session.run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    println!("{}", session.board);
}
