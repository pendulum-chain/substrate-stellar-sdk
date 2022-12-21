//! Stellar encoding of keys

use core::convert::{AsRef, TryInto};
use sp_std::vec::Vec;

use super::base32::{decode, encode};

use crate::StellarSdkError;

pub const ED25519_PUBLIC_KEY_BYTE_LENGTH: usize = 32;
pub const ED25519_PUBLIC_KEY_VERSION_BYTE: u8 = 6 << 3; // G

pub const ED25519_SECRET_SEED_BYTE_LENGTH: usize = 32;
pub const ED25519_SECRET_SEED_VERSION_BYTE: u8 = 18 << 3; // S

pub const MED25519_PUBLIC_KEY_BYTE_LENGTH: usize = 40;
pub const MED25519_PUBLIC_KEY_VERSION_BYTE: u8 = 12 << 3; // M

/// Use Stellar's key encoding to decode a key given as an ASCII string (as `&[u8]`)
pub fn decode_stellar_key<T: AsRef<[u8]>, const BYTE_LENGTH: usize>(
    encoded_key: T,
    version_byte: u8,
) -> Result<[u8; BYTE_LENGTH], StellarSdkError> {
    let decoded_array = decode(encoded_key.as_ref())?;
    if *encoded_key.as_ref() != encode(&decoded_array)[..] {
        return Err(StellarSdkError::InvalidStellarKeyEncoding)
    }

    let array_length = decoded_array.len();
    if array_length != 3 + BYTE_LENGTH {
        return Err(StellarSdkError::InvalidStellarKeyEncodingLength)
    }

    let crc_value = ((decoded_array[array_length - 1] as u16) << 8) | decoded_array[array_length - 2] as u16;
    let expected_crc_value = crc(&decoded_array[..array_length - 2]);
    if crc_value != expected_crc_value {
        return Err(StellarSdkError::InvalidStellarKeyChecksum { expected: expected_crc_value, found: crc_value })
    }

    let expected_version = version_byte;
    if decoded_array[0] != expected_version {
        return Err(StellarSdkError::InvalidStellarKeyEncodingVersion {
            expected_version: expected_version as char,
            found_version: decoded_array[0] as char,
        })
    }

    Ok(decoded_array[1..array_length - 2].try_into().unwrap())
}

/// Return the key encoding as an ASCII string (given as `Vec<u8>`)
pub fn encode_stellar_key<const BYTE_LENGTH: usize>(key: &[u8; BYTE_LENGTH], version_byte: u8) -> Vec<u8> {
    let mut unencoded_array = Vec::with_capacity(3 + BYTE_LENGTH);
    unencoded_array.push(version_byte);
    unencoded_array.extend(key.iter());

    let crc_value = crc(&unencoded_array);
    unencoded_array.push((crc_value & 0xff) as u8);
    unencoded_array.push((crc_value >> 8) as u8);

    encode(&unencoded_array)
}

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
