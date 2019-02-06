use uuid::Uuid;

#[derive(Clone, Copy, Debug)]
#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
#[derive(Deserialize, Serialize)]
pub struct UserId(pub Uuid);
