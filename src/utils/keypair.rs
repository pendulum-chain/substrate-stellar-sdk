use core::convert::TryInto;
use sp_std::{prelude::*, vec::Vec};

pub use sodalite::{
    sign_attached, sign_attached_open, sign_keypair_seed, Sign, SignPublicKey, SignSecretKey,
    SIGN_LEN, SIGN_PUBLIC_KEY_LEN, SIGN_SECRET_KEY_LEN,
};

use substrate_stellar_xdr::xdr;
use substrate_stellar_xdr::xdr_codec::XdrCodec;

use super::key_encoding::{
    binary_to_key_encoding, key_encoding_to_binary, EncodingVersion, KeyDecodeError,
};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct PublicKey(SignPublicKey);

impl PublicKey {
    fn from_binary(binary: [u8; 32]) -> PublicKey {
        PublicKey(binary)
    }

    fn from_encoded(encoded_key: &[u8]) -> Result<PublicKey, KeyDecodeError> {
        let binary = key_encoding_to_binary(EncodingVersion::Ed25519PublicKey, &encoded_key)?;
        if binary.len() != 32 {
            return Err(KeyDecodeError::InvalidEncodingLength);
        }
        let array: [u8; 32] = binary.try_into().unwrap();
        Ok(Self::from_binary(array))
    }

    fn get_encoded(&self) -> Vec<u8> {
        binary_to_key_encoding(EncodingVersion::Ed25519PublicKey, &self.0)
    }

    fn get_signature_hint(&self) -> [u8; 4] {
        let account_id_xdr = xdr::AccountId::PublicKeyTypeEd25519(self.0.clone())
            .to_xdr()
            .unwrap();

        account_id_xdr[account_id_xdr.len() - 4..]
            .try_into()
            .unwrap()
    }

    fn verify_signature(&self, message: &[u8], signature: &Sign) -> bool {
        let mut signed_message: Vec<u8> = Vec::with_capacity(message.len() + SIGN_LEN);

        signed_message.extend_from_slice(signature);
        signed_message.extend_from_slice(message);

        sign_attached_open(
            &mut vec![0; message.len() + SIGN_LEN],
            &signed_message,
            &self.0,
        )
        .is_ok()
    }
}

const SEED_LENGTH: usize = 32;
type Seed = [u8; SEED_LENGTH];

// the use of `secret` and `seed` is quite confusing
// `secret` (64 bit) is what tweetnacl calls the signing secret key
// `seed` (32 bit) is what Stellar calls the secret key
// we always have: secret = [..seed, ..public]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Keypair {
    public: PublicKey,
    secret: SignSecretKey,
    seed: Seed,
}

impl Keypair {
    fn from_seed(seed: &Seed) -> Keypair {
        let mut public_key: SignPublicKey = [0; SIGN_PUBLIC_KEY_LEN];
        let mut secret_key: SignSecretKey = [0; SIGN_SECRET_KEY_LEN];

        sign_keypair_seed(&mut public_key, &mut secret_key, seed);

        Keypair {
            public: PublicKey(public_key),
            secret: secret_key,
            seed: seed.clone(),
        }
    }

    fn from_encoded_secret(seed: &[u8]) -> Result<Keypair, KeyDecodeError> {
        let decoded_seed = key_encoding_to_binary(EncodingVersion::Ed25519SecretSeed, seed)?;

        Ok(Keypair::from_seed(
            &decoded_seed
                .try_into()
                .map_err(|_| KeyDecodeError::InvalidEncodingLength)?,
        ))
    }

    fn get_encoded_secret(&self) -> Vec<u8> {
        binary_to_key_encoding(EncodingVersion::Ed25519SecretSeed, &self.seed)
    }

    fn create_signature(&self, message: &[u8]) -> Sign {
        let mut signed_message: Vec<u8> = vec![0; message.len() + SIGN_LEN];
        sign_attached(&mut signed_message[..], message, &self.secret);

        signed_message.truncate(SIGN_LEN);
        signed_message.try_into().unwrap()
    }
}
