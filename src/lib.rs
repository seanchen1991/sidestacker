use structopt::StructOpt;

use error::Error;
use session::Session;

mod board;
mod error;
mod session;

#[derive(StructOpt, Debug)]
#[structopt(name = "sidestacker")]
pub enum SideStacker {
    /// Create a new SideStacker Session
    Create(Params),
    /// Connect to a SideStacker Session
    Connect(Params),
}

#[derive(StructOpt, Debug)]
#[structopt(about = "SideStacker parameters")]
pub struct Params {
    #[structopt(short, long, default_value = "0.0.0.0")]
    address: String,
    #[structopt(short, long, default_value = "8080")]
    port: u32,
}

/// Grabs CLI args and either creates a new game or connects to a pre-existing one.
pub fn init() -> Result<Session, Error> {
    // let session = match SideStacker::from_args() {
    //     SideStacker::Create(params) => Session::new(params),
    //     SideStacker::Connect(params) => Session::connect(params),
    // };

    Ok(Session::new())
}