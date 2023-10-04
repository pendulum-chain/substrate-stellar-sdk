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

/// Returns the hash of the Stellar object
pub trait ComputeHash {
    fn compute_hash(&self) -> Option<Hash>;
}

#[cfg(all(feature = "all-types", feature = "std"))]
impl ComputeHash for GeneralizedTransactionSet {
    fn compute_hash(&self) -> Option<Hash> {
        use crate::XdrCodec;
        use sha2::{Digest, Sha256};
        use std::convert::TryInto;

        let mut hasher = Sha256::new();
        hasher.update(self.to_xdr());

        hasher.finalize().as_slice().try_into().ok()
    }
}

#[cfg(all(feature = "all-types", feature = "std"))]
impl ComputeHash for TransactionSet {
    fn compute_hash(&self) -> Option<Hash> {
        use crate::XdrCodec;
        use sha2::{Digest, Sha256};
        use std::convert::TryInto;

        let mut hasher = Sha256::new();
        hasher.update(self.previous_ledger_hash);

        self.txes.get_vec().iter().for_each(|envlp| {
            hasher.update(envlp.to_xdr());
        });

        hasher.finalize().as_slice().try_into().ok()
    }
}