use sp_std::vec::Vec;

use super::base32::{decode, encode, Base32ParseError};

#[derive(Clone, Copy)]

struct EncodableKey<const N: usize, const M: u8>([u8; N]);

impl<const N: usize, const M: u8> EncodableKey<N, M> {
    pub fn binary_to_key_encoding(data: &[u8]) -> Result<Self, ()> {
        let mut unencoded_array = Vec::with_capacity(3 + data.len());
        unencoded_array.push(M);
        unencoded_array.extend(data.iter());

        let crc_value = crc(&unencoded_array[..]);
        unencoded_array.push((crc_value & 0xff) as u8);
        unencoded_array.push((crc_value >> 8) as u8);

        Self(encode(&unencoded_array).try_into()?)
    }
}

pub type Ed25519PublicKey = EncodableKey<32, { 6 << 3 }>;

pub enum EncodingVersion {
    Ed25519PublicKey,
    Ed25519SecretSeed,
    Med25519PublicKey,
    PreAuthTx,
    Sha256Hash,
}

fn get_encoding_version_byte(encoding_version: EncodingVersion) -> u8 {
    match encoding_version {
        Ed25519PublicKey => 6 << 3,   // G
        Ed25519SecretSeed => 18 << 3, // S
        Med25519PublicKey => 12 << 3, // M
        PreAuthTx => 19 << 3,         // T
        Sha256Hash => 23 << 3,        // X
    }
}

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

pub fn is_valid(encoding_version: EncodingVersion, encoded_key: &[u8]) -> bool {
    if encoded_key.len() != 56 {
        return false;
    }

    match key_encoding_to_binary(encoding_version, &encoded_key) {
        Ok(decoded) => decoded.len() == 32,
        _ => false,
    }
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

pub fn key_encoding_to_binary(
    encoding_version: EncodingVersion,
    encoded_key: &[u8],
) -> Result<Vec<u8>, KeyDecodeError> {
    let decoded_array = decode(encoded_key)?;
    if *encoded_key != encode(&decoded_array) {
        return Err(KeyDecodeError::InvalidEncoding);
    }

    let array_length = decoded_array.len();
    if array_length < 3 {
        return Err(KeyDecodeError::InvalidEncodingLength);
    }

    let crc_value =
        ((decoded_array[array_length - 1] as u16) << 8) | decoded_array[array_length - 2] as u16;
    let expected_crc_value = crc(&decoded_array[..array_length - 2]);
    if crc_value != expected_crc_value {
        return Err(KeyDecodeError::InvalidChecksum {
            expected: expected_crc_value,
            found: crc_value,
        });
    }

    let expected_version = get_encoding_version_byte(encoding_version);
    if decoded_array[0] != expected_version {
        return Err(KeyDecodeError::InvalidEncodingVersion {
            expected_version: expected_version as char,
            found_version: decoded_array[0] as char,
        });
    }

    Ok(Vec::<u8>::from(&decoded_array[1..array_length - 2]))
}

pub fn binary_to_key_encoding(encoding_version: EncodingVersion, data: &[u8]) -> Vec<u8> {
    let mut unencoded_array = Vec::with_capacity(3 + data.len());
    unencoded_array.push(get_encoding_version_byte(encoding_version));
    unencoded_array.extend(data.iter());

    let crc_value = crc(&unencoded_array[..]);
    unencoded_array.push((crc_value & 0xff) as u8);
    unencoded_array.push((crc_value >> 8) as u8);

    return encode(&unencoded_array);
}
