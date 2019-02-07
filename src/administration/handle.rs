use ed25519_dalek::PublicKey;

use administration::authentication::Credentials;
use administration::message::Request;
use administration::message::RequestBody;
use administration::message::Response;

/// Handle an incoming request.
pub fn authenticate_and_handle(pubk: &PublicKey, req: Request) -> Response {
    match req.token.verify(pubk) {
        Some(creds) => just_handle(&creds, &req.body),
        None => Response::BadAuthentication,
    }
}

/// Handle an incoming request, with the credentials already extracted from the
/// token. The credentials may be used for authorization.
pub fn just_handle(_creds: &Credentials, req: &RequestBody) -> Response {
    match req {
        RequestBody::Hello =>
            Response::Hello,
    }
}
