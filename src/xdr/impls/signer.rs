use crate::{IntoHash, IntoPublicKey, Signer, SignerKey, StellarSdkError};

impl Signer {
    pub fn from_ed25519_public_key<T: IntoPublicKey>(
        public_key: T,
        weight: u8,
    ) -> Result<Self, StellarSdkError> {
        Ok(Signer {
            key: SignerKey::from_ed25519_public_key(public_key)?,
            weight: weight as u32,
        })
    }

    pub fn from_pre_auth_tx<T: IntoHash>(hash: T, weight: u8) -> Result<Self, StellarSdkError> {
        Ok(Signer {
            key: SignerKey::from_pre_auth_tx(hash)?,
            weight: weight as u32,
        })
    }

    pub fn from_hash_x<T: IntoHash>(hash: T, weight: u8) -> Result<Self, StellarSdkError> {
        Ok(Signer {
            key: SignerKey::from_hash_x(hash)?,
            weight: weight as u32,
        })
    }
}
