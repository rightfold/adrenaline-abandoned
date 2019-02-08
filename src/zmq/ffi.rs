use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::raw::c_void;

#[link(name = "zmq")]
extern "C" {
    pub fn zmq_bind(socket: *mut c_void, endpoint: *const c_char) -> c_int;
    pub fn zmq_close(socket: *mut c_void) -> c_int;
    pub fn zmq_connect(socket: *mut c_void, endpoint: *const c_char) -> c_int;
    pub fn zmq_ctx_destroy(context: *mut c_void) -> c_int;
    pub fn zmq_ctx_new() -> *mut c_void;
    pub fn zmq_recv(socket: *mut c_void, buf: *mut c_void, len: usize,
                    flags: c_int) -> c_int;
    pub fn zmq_send(socket: *mut c_void, buf: *mut c_void, len: usize,
                    flags: c_int) -> c_int;
    pub fn zmq_socket(context: *mut c_void, type_: c_int) -> *mut c_void;
}
