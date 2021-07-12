use std::sync::Arc;

use structopt::StructOpt;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use server::{error::ServerError, process, Params, Server, Shared};

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    let Server::Start(Params {
        height,
        width,
        addr,
    }) = Server::from_args();

    let state = Arc::new(Mutex::new(Shared::try_new(height, width)?));
    let listener = TcpListener::bind(&addr).await?;

    println!("Server running on {}", addr);

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
