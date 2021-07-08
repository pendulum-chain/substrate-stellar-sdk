use crate::{AsHash, AsPublicKey, Error, PublicKey, SignerKey};

impl SignerKey {
    pub fn from_ed25519_public_key<T: AsPublicKey>(public_key: T) -> Result<Self, Error> {
        let public_key = public_key.as_public_key()?;
        let public_key = match public_key {
            PublicKey::PublicKeyTypeEd25519(public_key) => public_key,
        };

        Ok(Self::SignerKeyTypeEd25519(public_key))
    }

    pub fn from_pre_auth_tx<T: AsHash>(hash: T) -> Result<Self, Error> {
        Ok(Self::SignerKeyTypePreAuthTx(hash.as_hash()?))
    }

    pub fn from_hash_x<T: AsHash>(hash: T) -> Result<Self, Error> {
        Ok(Self::SignerKeyTypeHashX(hash.as_hash()?))
    }
}
