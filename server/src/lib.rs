use futures::{sink::SinkExt, StreamExt};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};
use tokio_util::codec::{Framed, LinesCodec};

use crate::error::ServerError;

pub mod error;

static DB_PATH: &str = "../db/games.db";

/// Sender half of the message channel.
type Tx = mpsc::UnboundedSender<String>;

/// Receiver half of the message channel.
type Rx = mpsc::UnboundedReceiver<String>;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "sidestacker-server",
    about = "Server for facillitating remote games of Sidestacker."
)]
pub enum Server {
    Start(Params),
}

/// CLI Params that the server accepts from the user.
#[derive(Debug, StructOpt)]
pub struct Params {
    /// The height of the game board.
    #[structopt(short, long, default_value = "7")]
    pub height: usize,
    /// The width of the game board.
    #[structopt(short, long, default_value = "7")]
    pub width: usize,
    /// The Address for the server to listen on.
    #[structopt(short, long, default_value = "0.0.0.0:8080")]
    pub addr: SocketAddr,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Player {
    /// First Player
    First,
    /// Second Player
    Second,
}

impl std::ops::Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Player::First => Player::Second,
            Player::Second => Player::First,
        }
    }
}

// TODO: Make this a `try_from`
impl From<u32> for Player {
    fn from(n: u32) -> Self {
        if n == 1 {
            Player::First
        } else {
            Player::Second
        }
    }
}

/// The sides from which Players may choose to insert a slot.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Side {
    Left,
    Right,
}

/// A Player's move.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Move {
    side: Side,
    row: usize,
}

/// A Player's turn.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Turn {
    source: Player,
    mov: Move,
}

/// Requests the server receives from clients.
#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    /// A client requests to join the game.
    Join,
    /// A client submits a `Turn` action.
    Turn(Turn),
}

/// The server's responses to client requests.
#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    /// There is enough capacity in the game. Tell the client which
    /// Player they are and the size of the board.
    Welcome { player: Player, height: usize, width: usize },
    /// There are enough Players for the game to start.
    GameStart,
    /// There is not enough capacity in the game.
    GameFull,
    /// A Player attempted to act out of turn.
    NotYourTurn,
    /// Server sends the current Player's Turn to the other Player.
    Turn(Turn),
    /// Server acknowledges a Player's proposed Turn.
    Acknowledged,
    /// The other Player disconnected.
    PlayerDisconnected,
    /// An internal server error occurred.
    ServerError,
}

/// Data and types that are shared between all peers playing the game.
pub struct Shared {
    /// Handle to the database.
    pub db_connection: Connection,
    /// Map of all Players and their send handles.
    pub players: HashMap<SocketAddr, Tx>,
    /// Indicates which Player's turn it is.
    pub current_player: Player,
    /// The Turns taken by the Players over the course of a game.
    pub turns: Vec<Turn>,
    /// The height of the game board.
    pub height: usize,
    /// The width of the game board.
    pub width: usize,
}

impl Shared {
    /// Attempt to create a new `Shared` instance.
    pub fn try_new(height: usize, width: usize) -> Result<Self, ServerError> {
        let db_connection = init_db()?;

        Ok(Shared {
            db_connection,
            players: HashMap::new(),
            current_player: Player::First,
            turns: Vec::new(),
            height,
            width,
        })
    }

    /// Send a line-encoded message to every peer except the sender.
    /// Reject the message if it isn't the current Player's turn.
    async fn broadcast(&mut self, sender: SocketAddr, message: &str) {
        for player in self.players.iter_mut() {
            if *player.0 != sender {
                let _ = player.1.send(message.into());
            }
        }
    }

    /// Send a line-encoded message back to the original sender.
    async fn back_to_sender(&mut self, sender: SocketAddr, message: &str) {
        let player = self.players.get_mut(&sender).unwrap();
        let _ = player.send(message.into());
    }
}

impl Drop for Shared {
    fn drop(&mut self) {
        println!("Saving game to database...");

        self.db_connection
            .execute(
                "INSERT INTO games (turns) values (?1)",
                &[&serde_json::to_string(&self.turns).expect("Failed to serialize Turns.")],
            )
            .expect("Error: Failed to persist game to database.");
    }
}

/// The state of each connected peer.
pub struct Peer {
    /// The Player's number, starting at 1.
    number: u32,
    /// The Peer's receiver handle.
    rx: Rx,
    /// Receive messages from players as lines, without having to worry
    /// about working at the raw byte level.
    lines: Framed<TcpStream, LinesCodec>,
}

impl Peer {
    /// Create a new `Peer` instance and notify the client.
    async fn new(
        state: Arc<Mutex<Shared>>,
        mut lines: Framed<TcpStream, LinesCodec>,
    ) -> Result<Option<Self>, ServerError> {
        let addr = lines.get_ref().peer_addr()?;
        let (tx, rx) = mpsc::unbounded_channel();

        let mut state = state.lock().await;
        let num_players = state.players.len() as u32 + 1;

        if num_players > 2 {
            let msg = serde_json::to_string(&Response::GameFull)?;
            state.back_to_sender(addr, &msg).await;
            return Ok(None);
        }

        state.players.insert(addr, tx);
        let height = state.height;
        let width = state.width;

        let player = Player::from(num_players);
        lines
            .send(serde_json::to_string(&Response::Welcome { player, height, width })?)
            .await?;

        Ok(Some(Peer {
            number: num_players,
            lines,
            rx,
        }))
    }
}

/// Process an individual player client.
pub async fn process(
    state: Arc<Mutex<Shared>>,
    stream: TcpStream,
    addr: SocketAddr,
) -> Result<(), ServerError> {
    let lines = Framed::new(stream, LinesCodec::new());

    let mut peer = match Peer::new(state.clone(), lines).await {
        Ok(peer) => match peer {
            Some(peer) => peer,
            None => return Err(ServerError::GameFull),
        },
        Err(e) => return Err(e),
    };

    // if there's currently only one Peer connected, prompt them to wait
    // until another Peer connects and the game can start

    // let everyone else know a new player has connected
    {
        let mut state = state.lock().await;
        state
            .broadcast(addr, &serde_json::to_string(&Response::GameStart)?)
            .await;
    }

    // Process incoming messages until stream is exhausted by a disconnect
    loop {
        tokio::select! {
            // A message was received from the other player. Send it to the current player.
            Some(msg) = peer.rx.recv() => {
                let mut state = state.lock().await;

                let turn: Turn = serde_json::from_str(&msg)?;
                state.turns.push(turn);

                peer.lines.send(serde_json::to_string(&Response::Turn(turn))?).await?;
            }

            result = peer.lines.next() => match result {
                // Message received from the current player.
                // Broadcast it to the other player.
                Some(Ok(msg)) => {
                    let mut state = state.lock().await;
                    let turn: Turn = serde_json::from_str(&msg)?;

                    if turn.source == state.current_player {
                        state.turns.push(turn);

                        state.broadcast(addr, &msg).await;
                        state.back_to_sender(addr, &serde_json::to_string(&Response::Acknowledged)?).await;

                        state.current_player = !state.current_player;
                    } else {
                        state.back_to_sender(addr, &serde_json::to_string(&Response::NotYourTurn)?).await;
                    }
                }
                // Some sort of error occurred
                Some(Err(e)) => {
                    let mut state = state.lock().await;

                    let error_message = format!("An error occurred while processing messages from Player {}: {}", peer.number, e);
                    eprintln!("{}", error_message);

                    state.back_to_sender(addr, &serde_json::to_string(&Response::ServerError)?).await;
                }
                // The stream has been exhausted
                None => break,
            }
        }
    }

    // A player disconnected!
    // Let the other player know.
    {
        let mut state = state.lock().await;
        state.players.remove(&addr);

        let msg = format!("Player {} has left the game.", peer.number);
        println!("{}", msg);

        state
            .broadcast(addr, &serde_json::to_string(&Response::PlayerDisconnected)?)
            .await;
    }

    Ok(())
}

/// Initialize a connection to the database.
pub fn init_db() -> Result<Connection, ServerError> {
    let connection = Connection::open(DB_PATH)?;

    if let Err(e) = connection.execute(
        "CREATE TABLE games (
            id INTEGER PRIMARY KEY,
            turns TEXT NOT NULL
        )",
        [],
    ) {
        eprintln!("Database error: {}", e);
    }

    Ok(connection)
}
