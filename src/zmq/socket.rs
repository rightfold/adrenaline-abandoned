use std::borrow::Cow;
use std::ffi::CStr;
use std::io;
use std::os::raw::c_int;
use std::os::raw::c_void;

use zmq::Context;
use zmq::ffi::*;

pub struct Socket(pub(super) *mut c_void);

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SocketType {
    REQ = 3,
    REP = 4,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Endpoint<'a> {
    Bind(Cow<'a, CStr>),
    Connect(Cow<'a, CStr>),
}

bitflags! {
    pub struct SendFlags: i32 {
        const DONTWAIT = 1;
        const SNDMORE  = 2;
    }
}

bitflags! {
    pub struct RecvFlags: i32 {
        const DONTWAIT = 1;
    }
}

impl Socket {
    pub fn new(context: &Context, type_: SocketType) -> io::Result<Self> {
        let raw = unsafe { zmq_socket(context.0, type_ as c_int) };
        if raw.is_null() {
            Err(io::Error::last_os_error())
        } else {
            Ok(Socket(raw))
        }
    }

    pub fn bind(&self, endpoint: &CStr) -> io::Result<()> {
        let status = unsafe { zmq_bind(self.0, endpoint.as_ptr()) };
        match status {
            0  => Ok(()),
            -1 => Err(io::Error::last_os_error()),
            s  => panic!("zmq_bind: Invalid status {}", s),
        }
    }

    pub fn connect(&self, endpoint: &CStr) -> io::Result<()> {
        let status = unsafe { zmq_connect(self.0, endpoint.as_ptr()) };
        match status {
            0  => Ok(()),
            -1 => Err(io::Error::last_os_error()),
            s  => panic!("zmq_bind: Invalid status {}", s),
        }
    }

    pub fn setup(&self, endpoint: &Endpoint) -> io::Result<()> {
        match endpoint {
            Endpoint::Bind(e) => self.bind(e),
            Endpoint::Connect(e) => self.connect(e),
        }
    }

    pub fn send(&self, message: &[u8], flags: SendFlags) -> io::Result<()> {
        let status = unsafe {
            zmq_send(
                self.0,
                message.as_ptr() as *mut c_void,
                message.len(),
                flags.bits,
            )
        };
        match status {
            -1 => Err(io::Error::last_os_error()),
            _  => Ok(()),
        }
    }

    pub fn recv(&self, message: &mut [u8], flags: RecvFlags) -> io::Result<usize> {
        let status = unsafe {
            zmq_recv(
                self.0,
                message.as_mut_ptr() as *mut c_void,
                message.len(),
                flags.bits
            )
        };
        match status {
            -1 => Err(io::Error::last_os_error()),
            n  => Ok(n as usize),
        }
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        let status = unsafe { zmq_close(self.0) };
        match status {
            0  => (),
            -1 => (),
            s  => panic!("zmq_close: Invalid status {}", s),
        }
    }
}

unsafe impl Send for Socket {}
