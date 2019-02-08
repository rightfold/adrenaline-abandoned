use std::io;

use administration::message::Request;
use administration::message::Response;
use log::log;
use zmq::RecvFlags;
use zmq::SendFlags;
use zmq::Socket;

pub const MAX_REQUEST_SIZE: usize = 16 * 1024;

pub fn serve<T, F>(rep: &Socket, mut f: F) -> io::Result<T>
    where F: FnMut(Request) -> Response {
    loop {
        exchange(rep, &mut f)?;
    }
}

pub fn exchange<F>(rep: &Socket, f: F) -> io::Result<()>
    where F: FnOnce(Request) -> Response {
    // TODO: Zeroing this buffer for every exchange is wasteful. Create it once
    // TODO: and reuse it.
    let mut req_bytes = [0; MAX_REQUEST_SIZE];
    let req_size = rep.recv(&mut req_bytes, RecvFlags::empty())?;
    log("request", req_size);

    let req = decode_request(&req_bytes, req_size);
    let res = req.map(f).unwrap_or_else(|r| r);

    let res_bytes = serde_cbor::to_vec::<Response>(&res).unwrap();
    rep.send(&res_bytes, SendFlags::empty())?;
    log("response", res_bytes.len());

    Ok(())
}

fn decode_request(bytes: &[u8], size: usize) -> Result<Request, Response> {
    if size > MAX_REQUEST_SIZE {
        return Err(Response::RequestTooLarge);
    }
    serde_cbor::from_slice::<Request>(&bytes[0 .. size])
        .map_err(|_| Response::MalformedRequest)
}
