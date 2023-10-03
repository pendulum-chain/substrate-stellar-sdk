use crate::{AsBinary, Hash, StellarSdkError};

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