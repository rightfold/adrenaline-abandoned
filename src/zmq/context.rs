use std::io;
use std::os::raw::c_void;

use zmq::ffi::*;

pub struct Context(pub(super) *mut c_void);

impl Context {
    pub fn new() -> io::Result<Self> {
        let raw = unsafe { zmq_ctx_new() };
        if raw.is_null() {
            Err(io::Error::last_os_error())
        } else {
            Ok(Context(raw))
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        let status = unsafe { zmq_ctx_destroy(self.0) };
        match status {
            0  => (),
            -1 => (), // TODO: If errno == EINTR, retry.
            s  => panic!("zmq_ctx_destroy: Invalid status {}", s),
        }
    }
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}
