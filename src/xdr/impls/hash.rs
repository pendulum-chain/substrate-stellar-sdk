use crate::{Error, Hash};
use core::convert::TryInto;

pub trait AsHash {
    fn as_hash(self) -> Result<Hash, Error>;
}

impl AsHash for Hash {
    fn as_hash(self) -> Result<Hash, Error> {
        Ok(self)
    }
}

impl AsHash for &str {
    fn as_hash(self) -> Result<Hash, Error> {
        let decoded = hex::decode(self).map_err(|err| Error::InvalidHexEncoding(err))?;
        let decoded_length = decoded.len();

        let hash: [u8; 32] = decoded.try_into().map_err(|_| Error::InvalidHashLength {
            found_length: decoded_length,
            expected_length: 32,
        })?;

        Ok(hash)
    }
}
