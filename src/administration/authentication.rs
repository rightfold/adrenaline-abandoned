use ed25519_dalek::PublicKey;
use ed25519_dalek::Signature;

use common::user::UserId;

/// A token is potentially invalid. To acquire the credentials, verification
/// must take place.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Token {
    pub signature: Signature,
    credentials: Credentials,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Credentials {
    pub user_id: UserId,
}

impl Token {
    pub fn verify(self, public_key: &PublicKey) -> Option<Credentials> {
        let signature = self.signature;
        let credentials = self.credentials;
        let message = serde_cbor::to_vec(&credentials).unwrap();
        public_key.verify(&message, &signature).ok().map(|()| credentials)
    }
}
