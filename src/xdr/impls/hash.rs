use crate::{AsBinary, Error, Hash};

pub trait IntoHash {
    fn into_hash(self) -> Result<Hash, Error>;
}

impl IntoHash for Hash {
    fn into_hash(self) -> Result<Hash, Error> {
        Ok(self)
    }
}

impl<T: AsRef<[u8]>> IntoHash for AsBinary<T> {
    fn into_hash(self) -> Result<Hash, Error> {
        self.as_binary()
    }
}
