use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;

use administration::handle::authenticate_and_handle;
use administration::network::serve;
use zmq::Context;
use zmq::Endpoint;
use zmq::Socket;
use zmq::SocketType;

#[derive(Deserialize)]
pub struct Config<'a> {
    pub endpoint: Endpoint<'a>,
}

impl<'a> Config<'a> {
    pub fn from_file<P>(path: P) -> Result<Self, Box<Error>>
        where P: AsRef<Path> {
        let file = File::open(path)?;
        let config = serde_json::from_reader(file)?;
        Ok(config)
    }
}

pub fn main() -> Result<(), Box<Error>> {
    let mut csrng = OsRng::new().unwrap();
    let keypair = Keypair::generate(&mut csrng);

    let config = Config::from_file(env::args_os().nth(1).unwrap())?;

    let ctx = Context::new()?;
    let rep = Socket::new(&ctx, SocketType::REP)?;
    rep.setup(&config.endpoint)?;

    serve(&rep, |req| authenticate_and_handle(&keypair.public, &req))?;

    Ok(())
}
