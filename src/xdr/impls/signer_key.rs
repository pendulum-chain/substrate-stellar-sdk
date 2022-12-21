use crate::{IntoHash, IntoPublicKey, SignerKey, StellarSdkError};

impl SignerKey {
    pub fn from_ed25519_public_key<T: IntoPublicKey>(public_key: T) -> Result<Self, StellarSdkError> {
        let public_key = public_key.into_public_key()?.into_binary();
        Ok(Self::SignerKeyTypeEd25519(public_key))
    }

    pub fn from_pre_auth_tx<T: IntoHash>(hash: T) -> Result<Self, StellarSdkError> {
        Ok(Self::SignerKeyTypePreAuthTx(hash.into_hash()?))
    }

    pub fn from_hash_x<T: IntoHash>(hash: T) -> Result<Self, StellarSdkError> {
        Ok(Self::SignerKeyTypeHashX(hash.into_hash()?))
    }
}
