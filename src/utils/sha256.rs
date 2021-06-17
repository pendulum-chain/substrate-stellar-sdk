use core::convert::{AsRef, TryInto};
use sha2::{Digest, Sha256};

pub type BinarySha256Hash = [u8; 32];

pub fn sha256<T: AsRef<[u8]>>(data: T) -> BinarySha256Hash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().as_slice().try_into().unwrap()
}
