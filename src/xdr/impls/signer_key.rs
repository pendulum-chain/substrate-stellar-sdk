use crate::{Error, IntoHash, IntoPublicKey, SignerKey};

impl SignerKey {
    pub fn from_ed25519_public_key<T: IntoPublicKey>(public_key: T) -> Result<Self, Error> {
        let public_key = public_key.into_public_key()?.into_binary();
        Ok(Self::SignerKeyTypeEd25519(public_key))
    }

    pub fn from_pre_auth_tx<T: IntoHash>(hash: T) -> Result<Self, Error> {
        Ok(Self::SignerKeyTypePreAuthTx(hash.into_hash()?))
    }

    pub fn from_hash_x<T: IntoHash>(hash: T) -> Result<Self, Error> {
        Ok(Self::SignerKeyTypeHashX(hash.into_hash()?))
    }
}
