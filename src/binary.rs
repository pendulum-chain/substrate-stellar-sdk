use core::convert::TryInto;

use crate::Error;

pub use AsBinary::{Binary, Hex};

pub enum AsBinary<T: AsRef<[u8]>> {
    Binary(T),
    Hex(T),
}

impl<T: AsRef<[u8]>> AsBinary<T> {
    pub fn as_binary<const N: usize>(self) -> Result<[u8; N], Error> {
        match self {
            Binary(binary) => {
                let binary = binary.as_ref();

                binary.try_into().map_err(|_| Error::InvalidBinaryLength {
                    found_length: binary.len(),
                    expected_length: N,
                })
            }

            Hex(hex) => {
                let decoded = hex::decode(hex).map_err(|err| Error::InvalidHexEncoding(err))?;
                let decoded_length = decoded.len();

                decoded.try_into().map_err(|_| Error::InvalidBinaryLength {
                    found_length: decoded_length,
                    expected_length: N,
                })
            }
        }
    }
}
