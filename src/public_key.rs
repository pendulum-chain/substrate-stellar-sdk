use core::convert::TryInto;

use sp_std::{vec, vec::Vec};

use crate::{
    utils::key_encoding::{
        decode_stellar_key, encode_stellar_key, ED25519_PUBLIC_KEY_BYTE_LENGTH,
        ED25519_PUBLIC_KEY_VERSION_BYTE,
    },
    PublicKey, StellarSdkError, XdrCodec,
};

use sodalite::{sign_attached_open, Sign as Signature, SIGN_LEN};

pub trait IntoPublicKey {
    fn into_public_key(self) -> Result<PublicKey, StellarSdkError>;
}

impl IntoPublicKey for PublicKey {
    fn into_public_key(self) -> Result<PublicKey, StellarSdkError> {
        Ok(self)
    }
}

impl<T: AsRef<[u8]>> IntoPublicKey for T {
    fn into_public_key(self) -> Result<PublicKey, StellarSdkError> {
        PublicKey::from_encoding(self)
    }
}

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

    pub fn from_encoding<T: AsRef<[u8]>>(encoded_key: T) -> Result<Self, StellarSdkError> {
        let decoded_key = decode_stellar_key(encoded_key, ED25519_PUBLIC_KEY_VERSION_BYTE)?;
        Ok(Self::from_binary(decoded_key))
    }

    /// Return the key encoding as an ASCII string (given as `Vec<u8>`)
    pub fn to_encoding(&self) -> Vec<u8> {
        let key = self.as_binary();
        encode_stellar_key(key, ED25519_PUBLIC_KEY_VERSION_BYTE)
    }

    pub fn get_signature_hint(&self) -> [u8; 4] {
        let account_id_xdr = Self::PublicKeyTypeEd25519(self.as_binary().clone()).to_xdr();

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
