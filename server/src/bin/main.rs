use std::sync::Arc;

use structopt::StructOpt;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use server::{error::ServerError, process, Server, Shared};

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    let state = Arc::new(Mutex::new(Shared::try_new()?));
    let Server::Start(params) = Server::from_args();
    let listener = TcpListener::bind(&params.addr).await?;

    println!("Server running on {}", params.addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            println!("Got a connection");

            if let Err(e) = process(state, stream, addr).await {
                eprintln!("Error: {}", e);
            }
        });
    }
}
