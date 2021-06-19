//! Stellar encoding of keys

use core::convert::{AsRef, TryInto};
use sp_std::vec::Vec;

use super::base32::{decode, encode, Base32ParseError};

#[derive(Clone, Eq, PartialEq, Debug)]

/// The basic type for a key that implements Stellar's key encoding
///
/// The `BYTE_LENGTH` parameter determines the byte length of the key.
/// The `VERSION_BYTE` determines the prefix byte used for the encoding. It is used
/// to easily distinguish different kinds of keys.
/// A key encoding is an ASCII string.
pub struct EncodableKey<const BYTE_LENGTH: usize, const VERSION_BYTE: u8>([u8; BYTE_LENGTH]);

impl<const BYTE_LENGTH: usize, const VERSION_BYTE: u8> EncodableKey<BYTE_LENGTH, VERSION_BYTE> {
    /// Create a new `EncodableKey` from a raw binary key
    pub fn from_binary(binary: [u8; BYTE_LENGTH]) -> Self {
        EncodableKey(binary)
    }

    /// Return the raw binary key
    pub fn get_binary(&self) -> &[u8; BYTE_LENGTH] {
        &self.0
    }

    /// Check whether a given ASCII string (given as `&[u8]`) represents a valid encoding
    pub fn is_valid<T: AsRef<[u8]>>(encoded_key: T) -> bool {
        match Self::from_encoding(encoded_key) {
            Ok(decoded) => decoded.0.len() == BYTE_LENGTH,
            _ => false,
        }
    }

    /// Create a new `EncodableKey` from a key encoding given as an ASCII string (as `&[u8]`)
    pub fn from_encoding<T: AsRef<[u8]>>(encoded_key: T) -> Result<Self, KeyDecodeError> {
        let decoded_array = decode(encoded_key.as_ref())?;
        if *encoded_key.as_ref() != encode(&decoded_array)[..] {
            return Err(KeyDecodeError::InvalidEncoding);
        }

        let array_length = decoded_array.len();
        if array_length != 3 + BYTE_LENGTH {
            return Err(KeyDecodeError::InvalidEncodingLength);
        }

        let crc_value = ((decoded_array[array_length - 1] as u16) << 8)
            | decoded_array[array_length - 2] as u16;
        let expected_crc_value = crc(&decoded_array[..array_length - 2]);
        if crc_value != expected_crc_value {
            return Err(KeyDecodeError::InvalidChecksum {
                expected: expected_crc_value,
                found: crc_value,
            });
        }

        let expected_version = VERSION_BYTE;
        if decoded_array[0] != expected_version {
            return Err(KeyDecodeError::InvalidEncodingVersion {
                expected_version: expected_version as char,
                found_version: decoded_array[0] as char,
            });
        }

        Ok(Self(decoded_array[1..array_length - 2].try_into().unwrap()))
    }

    /// Return the key encoding as an ASCII string (given as `Vec<u8>`)
    pub fn to_encoding(&self) -> Vec<u8> {
        let mut unencoded_array = Vec::with_capacity(3 + BYTE_LENGTH);
        unencoded_array.push(VERSION_BYTE);
        unencoded_array.extend(self.0.iter());

        let crc_value = crc(&unencoded_array);
        unencoded_array.push((crc_value & 0xff) as u8);
        unencoded_array.push((crc_value >> 8) as u8);

        encode(&unencoded_array)
    }
}

/// The public key of an Ed25519 signing keypair
pub type Ed25519PublicKey = EncodableKey<32, { 6 << 3 } /* G */>;

/// The secret seed of an Ed25519 signing keypair
pub type Ed25519SecretSeed = EncodableKey<32, { 18 << 3 } /* S */>;

#[allow(dead_code)]
pub type Med25519PublicKey = EncodableKey<40, { 12 << 3 } /* M */>;
#[allow(dead_code)]
pub type PreAuthTx = EncodableKey<32, { 19 << 3 } /* T */>;
#[allow(dead_code)]
pub type Sha256Hash = EncodableKey<32, { 23 << 3 } /* X */>;

fn crc<T: AsRef<[u8]>>(byte_array: T) -> u16 {
    let mut crc: u16 = 0;

    for byte in byte_array.as_ref().iter() {
        let mut code: u16 = crc >> 8 & 0xff;

        code ^= *byte as u16;
        code ^= code >> 4;
        crc = (crc << 8) & 0xffff;
        crc ^= code;
        code = (code << 5) & 0xffff;
        crc ^= code;
        code = (code << 7) & 0xffff;
        crc ^= code;
    }

    crc
}

/// Error type for key decoding errors
#[derive(Debug)]
pub enum KeyDecodeError {
    /// The encoding can be decoded but is not the canonical encoding of the underlying binary key
    InvalidEncoding,

    /// The encoding has an invalid length
    InvalidEncodingLength,

    /// The initial version byte is invalid for this `EncodableKey`
    InvalidEncodingVersion {
        expected_version: char,
        found_version: char,
    },

    /// The checksum in the encoding is invaliid
    InvalidChecksum { expected: u16, found: u16 },

    /// The base32 encoding is invalid
    InvalidBase32(Base32ParseError),
}

impl From<Base32ParseError> for KeyDecodeError {
    fn from(error: Base32ParseError) -> Self {
        KeyDecodeError::InvalidBase32(error)
    }
}
