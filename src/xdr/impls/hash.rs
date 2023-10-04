use crate::{AsBinary, Hash, StellarSdkError};

#[cfg(all(feature = "all-types", feature = "std"))]
use crate::types::{GeneralizedTransactionSet, TransactionSet};

pub trait IntoHash {
    fn into_hash(self) -> Result<Hash, StellarSdkError>;
}

impl IntoHash for Hash {
    fn into_hash(self) -> Result<Hash, StellarSdkError> {
        Ok(self)
    }
}

impl<T: AsRef<[u8]>> IntoHash for AsBinary<T> {
    fn into_hash(self) -> Result<Hash, StellarSdkError> {
        self.as_binary()
    }
}

#[cfg(all(feature = "all-types", feature = "std"))]
impl IntoHash for GeneralizedTransactionSet {
    fn into_hash(self) -> Result<Hash, StellarSdkError> {
        use crate::XdrCodec;
        use sha2::{Digest, Sha256};
        use std::convert::TryInto;

        let mut hasher = Sha256::new();
        hasher.update(self.to_xdr());

        hasher
            .finalize()
            .as_slice()
            .try_into()
            .map_err(|e: std::array::TryFromSliceError| StellarSdkError::InvalidHashConversion(e.to_string()))
    }
}

#[cfg(all(feature = "all-types", feature = "std"))]
impl IntoHash for TransactionSet {
    fn into_hash(self) -> Result<Hash, StellarSdkError> {
        use crate::XdrCodec;
        use sha2::{Digest, Sha256};
        use std::convert::TryInto;

        let mut hasher = Sha256::new();
        hasher.update(self.previous_ledger_hash);

        self.txes.get_vec().iter().for_each(|envlp| {
            hasher.update(envlp.to_xdr());
        });

        hasher
            .finalize()
            .as_slice()
            .try_into()
            .map_err(|e: std::array::TryFromSliceError| StellarSdkError::InvalidHashConversion(e.to_string()))
    }
}
