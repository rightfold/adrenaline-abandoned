use administration::authentication::Token;

/// A request contains the actual request (known as the request body) and the
/// token used for authentication.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Request {
    pub token: Token,
    pub body: RequestBody,
}

/// The request body contains the actual request.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RequestBody {
    Hello,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Response {
    Hello,

    MalformedRequest,
    RequestTooLarge,
    Unauthorized,
}
