//! Ed25519 keypairs and public keys

use core::convert::{AsRef, TryInto};
use sp_std::{prelude::*, vec, vec::Vec};

use sodalite::{
    sign_attached, sign_attached_open, sign_keypair_seed, SignPublicKey, SignSecretKey, SIGN_LEN,
    SIGN_PUBLIC_KEY_LEN, SIGN_SECRET_KEY_LEN,
};

use crate::{
    types::{AccountId, Curve25519Secret, PublicKey},
    utils::key_encoding::{
        decode_stellar_key, encode_stellar_key, ED25519_PUBLIC_KEY_BYTE_LENGTH,
        ED25519_PUBLIC_KEY_VERSION_BYTE, ED25519_SECRET_SEED_BYTE_LENGTH,
        ED25519_SECRET_SEED_VERSION_BYTE,
    },
    XdrCodec,
};

use super::utils::key_encoding::KeyDecodeError;
pub use sodalite::Sign as Signature;

/// The public key of an Ed25519 signing key pair
///
/// This type is also used for Stellar account ids.
/// ```
/// let public = "GBIVKYSF6RP4U57KPZ524X47NGTQYYPZAZ4UX5ZFYAYBJWRFXHKHDQVL";
/// let public_key = substrate_stellar_sdk::types::PublicKey::from_encoding(public);
/// assert!(public_key.is_ok());
/// let public_key = public_key.unwrap();
/// assert_eq!(&public_key.to_encoding().as_slice(), &public.as_bytes());
/// ```
impl PublicKey {
    pub fn from_binary(binary: [u8; ED25519_PUBLIC_KEY_BYTE_LENGTH]) -> Self {
        PublicKey::PublicKeyTypeEd25519(binary)
    }

    /// Return the raw binary key as a reference
    pub fn as_binary(&self) -> &[u8; ED25519_PUBLIC_KEY_BYTE_LENGTH] {
        match self {
            PublicKey::PublicKeyTypeEd25519(key) => key,
        }
    }

    /// Turn into the raw binary key
    pub fn into_binary(self) -> [u8; ED25519_PUBLIC_KEY_BYTE_LENGTH] {
        *self.as_binary()
    }

    pub fn from_encoding<T: AsRef<[u8]>>(encoded_key: T) -> Result<Self, KeyDecodeError> {
        let decoded_key = decode_stellar_key(encoded_key, ED25519_PUBLIC_KEY_VERSION_BYTE)?;
        Ok(Self::from_binary(decoded_key))
    }

    /// Return the key encoding as an ASCII string (given as `Vec<u8>`)
    pub fn to_encoding(&self) -> Vec<u8> {
        let key = self.as_binary();
        encode_stellar_key(key, ED25519_PUBLIC_KEY_VERSION_BYTE)
    }

    pub fn get_signature_hint(&self) -> [u8; 4] {
        let account_id_xdr = AccountId::PublicKeyTypeEd25519(self.as_binary().clone()).to_xdr();

        account_id_xdr[account_id_xdr.len() - 4..]
            .try_into()
            .unwrap()
    }

    /// Verify the signature of a message.
    ///
    /// Given the raw binary `message`, check whether the raw binary `signature` is valid.
    pub fn verify_signature<T: AsRef<[u8]>>(&self, message: T, signature: &Signature) -> bool {
        let message = message.as_ref();
        let mut signed_message: Vec<u8> = Vec::with_capacity(message.len() + SIGN_LEN);

        signed_message.extend_from_slice(signature);
        signed_message.extend_from_slice(message);

        sign_attached_open(
            &mut vec![0; message.len() + SIGN_LEN],
            &signed_message,
            self.as_binary(),
        )
        .is_ok()
    }
}

/// An Ed25519 signing keypair
///
/// This type is used for signing Stellar transactions.
/// ```
/// let secret = "SCDSVACTNFNSD5LQZ5LWUWEY3UIAML2J7ALPFCD6ZX4D3TVJV7X243N3";
/// let public = "GBIVKYSF6RP4U57KPZ524X47NGTQYYPZAZ4UX5ZFYAYBJWRFXHKHDQVL";
/// let secret_key = substrate_stellar_sdk::keypair::SecretKey::from_encoding(secret);
/// assert!(secret_key.is_ok());
/// let secret_key = secret_key.unwrap();
/// assert_eq!(&secret_key.to_encoding().as_slice(), &secret.as_bytes());
/// assert_eq!(&secret_key.get_encoded_public().as_slice(), &public.as_bytes());
/// ```
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SecretKey {
    // the use of `signer_key` and `secret_seed` is quite confusing
    // `signer_key` (512 bit) is what tweetnacl calls the signing secret key
    // `secret_seed` (256 bit) is what Stellar calls the secret key
    // we always have: signer_key = [..secret_seed, ..public] (in JS notation)
    public: PublicKey,
    secret_seed: Curve25519Secret,
    signer_key: SignSecretKey,
}

impl SecretKey {
    /// Generate a new keypair from a raw binary secret seed
    pub fn from_binary(seed: [u8; ED25519_SECRET_SEED_BYTE_LENGTH]) -> SecretKey {
        let mut public_key: SignPublicKey = [0; SIGN_PUBLIC_KEY_LEN];
        let mut secret_key: SignSecretKey = [0; SIGN_SECRET_KEY_LEN];

        sign_keypair_seed(&mut public_key, &mut secret_key, &seed);

        SecretKey {
            public: PublicKey::from_binary(public_key),
            secret_seed: Curve25519Secret { key: seed },
            signer_key: secret_key,
        }
    }

    /// Return the raw binary key as a reference
    pub fn as_binary(&self) -> &[u8; ED25519_SECRET_SEED_BYTE_LENGTH] {
        &self.secret_seed.key
    }

    /// Turn into the raw binary key
    pub fn into_binary(self) -> [u8; ED25519_SECRET_SEED_BYTE_LENGTH] {
        *self.as_binary()
    }

    pub fn from_encoding<T: AsRef<[u8]>>(encoded_key: T) -> Result<Self, KeyDecodeError> {
        let decoded_key = decode_stellar_key(encoded_key, ED25519_SECRET_SEED_VERSION_BYTE)?;
        Ok(Self::from_binary(decoded_key))
    }

    /// Return the key encoding as an ASCII string (given as `Vec<u8>`)
    pub fn to_encoding(&self) -> Vec<u8> {
        let key = self.as_binary();
        encode_stellar_key(key, ED25519_SECRET_SEED_VERSION_BYTE)
    }

    /// Return the encoded public key
    pub fn get_encoded_public(&self) -> Vec<u8> {
        self.public.to_encoding()
    }

    /// Return the public key of the keypair as a `PublicKey`
    pub fn get_public(&self) -> &PublicKey {
        &self.public
    }

    /// Create a signature for the `message`
    pub fn create_signature<T: AsRef<[u8]>>(&self, message: T) -> Signature {
        let message = message.as_ref();
        let mut signed_message: Vec<u8> = vec![0; message.len() + SIGN_LEN];
        sign_attached(&mut signed_message[..], message, &self.signer_key);

        signed_message.truncate(SIGN_LEN);
        signed_message.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::keypair::{PublicKey, SecretKey};

    #[test]
    fn keypair() {
        let secret = "SCDSVACTNFNSD5LQZ5LWUWEY3UIAML2J7ALPFCD6ZX4D3TVJV7X243N3";
        let public = "GBIVKYSF6RP4U57KPZ524X47NGTQYYPZAZ4UX5ZFYAYBJWRFXHKHDQVL";
        let keypair = SecretKey::from_encoding(secret);
        assert!(keypair.is_ok());
        let keypair = keypair.unwrap();
        assert_eq!(&keypair.to_encoding().as_slice(), &secret.as_bytes());
        assert_eq!(&keypair.get_encoded_public().as_slice(), &public.as_bytes());
    }

    #[test]
    fn public_key() {
        let public = "GBIVKYSF6RP4U57KPZ524X47NGTQYYPZAZ4UX5ZFYAYBJWRFXHKHDQVL";
        let public_key = PublicKey::from_encoding(public);
        assert!(public_key.is_ok());
        let public_key = public_key.unwrap();
        assert_eq!(&public_key.to_encoding().as_slice(), &public.as_bytes());
    }
}
