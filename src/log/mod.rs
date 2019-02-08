use serde::Serialize;
use std::io;
use std::io::Write;

pub fn log<M>(key: &str, msg: M) where M: Serialize {
    let w = io::stderr();
    let mut ww = w.lock();
    let _ = write!(&mut ww, "{} ", key);
    let _ = serde_json::to_writer(&mut ww, &msg);
    let _ = write!(&mut ww, "\n");
}
