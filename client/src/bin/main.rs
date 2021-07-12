use futures::StreamExt;

use structopt::StructOpt;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LinesCodec};

use client::{error::ClientError, process, session::Session, Client, Connection, Params, Response};

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    let Client::Connect(Params { addr }) = Client::from_args();

    let mut connection = match TcpStream::connect(addr).await {
        Ok(stream) => Connection {
            lines: Framed::new(stream, LinesCodec::new()),
        },
        Err(e) => return Err(ClientError::ConnectionError(e.to_string())),
    };

    println!("Client connected to server at {}", addr);

    let response = match connection.lines.next().await {
        Some(Ok(resp)) => resp,
        Some(Err(e)) => return Err(ClientError::ServerError(e.to_string())),
        None => {
            return Err(ClientError::ServerError(String::from(
                "No response from server.",
            )))
        }
    };

    let response: Response = serde_json::from_str(&response)?;

    let mut session = match response {
        Response::Welcome {
            player,
            height,
            width,
        } => Session::new(player, height, width),
        _ => {
            return Err(ClientError::ServerError(String::from(
                "Inappropriate response from server.",
            )))
        }
    };

    loop {
        tokio::spawn(async move {
            if let Err(e) = process(&mut session, &mut connection).await {
                eprintln!("Error: {}", e);
            }
        });
    }
}
