use ed25519_dalek::Keypair;
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Credentials {
    pub user_id: UserId,
}

impl Token {
    pub fn verify(&self, public_key: &PublicKey) -> Option<&Credentials> {
        let signature = &self.signature;
        let credentials = &self.credentials;
        let message = serde_cbor::to_vec(&credentials).unwrap();
        public_key.verify(&message, signature).ok().map(|()| credentials)
    }
}

impl Credentials {
    pub fn sign(self, keypair: &Keypair) -> Token {
        let message = serde_cbor::to_vec(&self).unwrap();
        let signature = keypair.sign(&message);
        Token{signature, credentials: self}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::rngs::OsRng;
    use uuid::Uuid;

    fn test_data() -> (Keypair, Credentials) {
        let mut csrng = OsRng::new().unwrap();
        let keypair = Keypair::generate(&mut csrng);

        let user_id = UserId(Uuid::new_v4());
        let creds = Credentials{user_id};

        (keypair, creds)
    }

    #[test]
    fn test_verify_valid_token() {
        let (keypair, creds) = test_data();
        let token = creds.clone().sign(&keypair);
        assert_eq!(token.verify(&keypair.public), Some(&creds));
    }

    #[test]
    fn test_verify_invalid_token() {
        let (keypair1, creds) = test_data();
        let (keypair2, _    ) = test_data();
        let token = creds.sign(&keypair1);
        assert_eq!(token.verify(&keypair2.public), None);
    }
}
