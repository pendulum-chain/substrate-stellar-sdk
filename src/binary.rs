use sp_std::vec::Vec;

use core::convert::TryInto;

use crate::StellarSdkError;

pub enum AsBinary<T: AsRef<[u8]>> {
    Binary(T),
    Hex(T),
}

impl<T: AsRef<[u8]>> AsBinary<T> {
    pub fn as_binary<const N: usize>(self) -> Result<[u8; N], StellarSdkError> {
        match self {
            AsBinary::Binary(binary) => {
                let binary = binary.as_ref();

                binary.try_into().map_err(|_| StellarSdkError::InvalidBinaryLength {
                    found_length: binary.len(),
                    expected_length: N,
                })
            },

            AsBinary::Hex(hex) => {
                let hex = hex.as_ref();
                let mut decoded: Vec<u8> = Vec::with_capacity(hex.len() / 2);
                decoded.resize(hex.len() / 2, 0);
                let decoded_length = decoded.len();

                decoded.try_into().map_err(|_| StellarSdkError::InvalidBinaryLength {
                    found_length: decoded_length,
                    expected_length: N,
                })
            },
        }
    }
}
