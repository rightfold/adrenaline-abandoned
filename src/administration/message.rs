#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Request {
    Hello,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Response {
    Hello,

    MalformedRequest,
    RequestTooLarge,
}
