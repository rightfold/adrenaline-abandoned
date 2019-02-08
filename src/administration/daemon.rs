use std::env;
use std::error::Error;
use std::fs::File;

use administration::handle::handle;
use administration::network::serve;
use log::log;
use zmq::Context;
use zmq::Endpoint;
use zmq::Socket;
use zmq::SocketType;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config<'a> {
    pub endpoint: Endpoint<'a>,
}

pub fn main() -> Result<(), Box<Error>> {
    let config_path = env::args_os().nth(1).unwrap();
    let config_file = File::open(config_path)?;
    let config: Config = serde_json::from_reader(config_file)?;
    log("read_config", &config);

    let endpoint = config.endpoint;

    let ctx = Context::new()?;
    let rep = Socket::new(&ctx, SocketType::REP)?;
    rep.setup(&endpoint)?;
    log("setup_socket", &endpoint);

    serve(&rep, |req| handle(&req))?;

    Ok(())
}
