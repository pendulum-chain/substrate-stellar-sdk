use crate::{AsHash, AsPublicKey, Error, Signer, SignerKey};

impl Signer {
    pub fn from_ed25519_public_key<T: AsPublicKey>(
        public_key: T,
        weight: u8,
    ) -> Result<Self, Error> {
        Ok(Signer {
            key: SignerKey::from_ed25519_public_key(public_key)?,
            weight: weight as u32,
        })
    }

    pub fn from_pre_auth_tx<T: AsHash>(hash: T, weight: u8) -> Result<Self, Error> {
        Ok(Signer {
            key: SignerKey::from_pre_auth_tx(hash)?,
            weight: weight as u32,
        })
    }

    pub fn from_hash_x<T: AsHash>(hash: T, weight: u8) -> Result<Self, Error> {
        Ok(Signer {
            key: SignerKey::from_hash_x(hash)?,
            weight: weight as u32,
        })
    }
}
