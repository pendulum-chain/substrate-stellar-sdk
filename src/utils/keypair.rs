use core::convert::TryInto;
use sp_std::{prelude::*, vec::Vec};

pub use sodalite::{
    sign_attached, sign_attached_open, sign_keypair_seed, Sign as Signature, SignPublicKey,
    SignSecretKey, SIGN_LEN, SIGN_PUBLIC_KEY_LEN, SIGN_SECRET_KEY_LEN,
};

use substrate_stellar_xdr::xdr;
use substrate_stellar_xdr::xdr_codec::XdrCodec;

use super::key_encoding::{Ed25519PublicKey, Ed25519SecretSeed, KeyDecodeError};

pub type PublicKey = Ed25519PublicKey;

impl PublicKey {
    fn get_signature_hint(&self) -> [u8; 4] {
        let account_id_xdr = xdr::AccountId::PublicKeyTypeEd25519(self.get_binary().clone())
            .to_xdr()
            .unwrap();

        account_id_xdr[account_id_xdr.len() - 4..]
            .try_into()
            .unwrap()
    }

    fn verify_signature(&self, message: &[u8], signature: &Signature) -> bool {
        let mut signed_message: Vec<u8> = Vec::with_capacity(message.len() + SIGN_LEN);

        signed_message.extend_from_slice(signature);
        signed_message.extend_from_slice(message);

        sign_attached_open(
            &mut vec![0; message.len() + SIGN_LEN],
            &signed_message,
            self.get_binary(),
        )
        .is_ok()
    }
}

// the use of `secret` and `seed` is quite confusing
// `secret` (64 bit) is what tweetnacl calls the signing secret key
// `seed` (32 bit) is what Stellar calls the secret key
// we always have: secret = [..seed, ..public]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Keypair {
    public: PublicKey,
    secret_seed: Ed25519SecretSeed,
    signer_key: SignSecretKey,
}

impl Keypair {
    fn from_binary_secret(seed: Ed25519SecretSeed) -> Keypair {
        let mut public_key: SignPublicKey = [0; SIGN_PUBLIC_KEY_LEN];
        let mut secret_key: SignSecretKey = [0; SIGN_SECRET_KEY_LEN];

        sign_keypair_seed(&mut public_key, &mut secret_key, seed.get_binary());

        Keypair {
            public: PublicKey::from_binary(public_key),
            secret_seed: seed,
            signer_key: secret_key,
        }
    }

    fn from_encoded_secret(encoded_seed: &[u8]) -> Result<Keypair, KeyDecodeError> {
        let binary_seed = Ed25519SecretSeed::from_encoding(&encoded_seed)?;

        Ok(Keypair::from_binary_secret(binary_seed))
    }

    fn get_encoded_secret(&self) -> Vec<u8> {
        self.secret_seed.to_encoding()
    }

    fn create_signature(&self, message: &[u8]) -> Signature {
        let mut signed_message: Vec<u8> = vec![0; message.len() + SIGN_LEN];
        sign_attached(&mut signed_message[..], message, &self.signer_key);

        signed_message.truncate(SIGN_LEN);
        signed_message.try_into().unwrap()
    }
}
