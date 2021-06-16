use core::convert::TryInto;
use sp_std::vec::Vec;

use super::base32::{decode, encode, Base32ParseError};

#[derive(Clone, Eq, PartialEq, Debug)]

pub struct EncodableKey<const ByteLength: usize, const VersionByte: u8>([u8; ByteLength]);

impl<const ByteLength: usize, const VersionByte: u8> EncodableKey<ByteLength, VersionByte> {
    pub fn from_binary(binary: [u8; ByteLength]) -> Self {
        EncodableKey(binary)
    }

    pub fn get_binary(&self) -> &[u8; ByteLength] {
        &self.0
    }

    pub fn is_valid(encoded_key: &[u8]) -> bool {
        match Self::from_encoding(encoded_key) {
            Ok(decoded) => decoded.0.len() == ByteLength,
            _ => false,
        }
    }

    pub fn from_encoding(encoded_key: &[u8]) -> Result<Self, KeyDecodeError> {
        let decoded_array = decode(encoded_key)?;
        if *encoded_key != encode(&decoded_array) {
            return Err(KeyDecodeError::InvalidEncoding);
        }

        let array_length = decoded_array.len();
        if array_length != 3 + ByteLength {
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

        let expected_version = VersionByte;
        if decoded_array[0] != expected_version {
            return Err(KeyDecodeError::InvalidEncodingVersion {
                expected_version: expected_version as char,
                found_version: decoded_array[0] as char,
            });
        }

        Ok(Self(decoded_array[1..array_length - 2].try_into().unwrap()))
    }

    pub fn to_encoding(&self) -> Vec<u8> {
        let mut unencoded_array = Vec::with_capacity(3 + ByteLength);
        unencoded_array.push(VersionByte);
        unencoded_array.extend(self.0.iter());

        let crc_value = crc(&unencoded_array[..]);
        unencoded_array.push((crc_value & 0xff) as u8);
        unencoded_array.push((crc_value >> 8) as u8);

        encode(&unencoded_array)
    }
}

pub type Ed25519PublicKey = EncodableKey<32, { 6 << 3 } /* G */>;
pub type Ed25519SecretSeed = EncodableKey<32, { 18 << 3 } /* S */>;
pub type Med25519PublicKey = EncodableKey<40, { 12 << 3 } /* M */>;
pub type PreAuthTx = EncodableKey<32, { 19 << 3 } /* T */>;
pub type Sha256Hash = EncodableKey<32, { 23 << 3 } /* X */>;

fn crc(byteArray: &[u8]) -> u16 {
    let mut crc: u16 = 0;

    for byte in byteArray.iter() {
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

pub enum KeyDecodeError {
    InvalidEncoding,
    InvalidEncodingLength,
    InvalidEncodingVersion {
        expected_version: char,
        found_version: char,
    },
    InvalidChecksum {
        expected: u16,
        found: u16,
    },
    InvalidBase32(Base32ParseError),
}

impl From<Base32ParseError> for KeyDecodeError {
    fn from(error: Base32ParseError) -> Self {
        KeyDecodeError::InvalidBase32(error)
    }
}
