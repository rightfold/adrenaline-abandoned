use administration::message::Request;
use administration::message::Response;

/// Handle an incoming request.
pub fn handle(req: &Request) -> Response {
    match req {
        Request::Hello =>
            Response::Hello,
    }
}
