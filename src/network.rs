//! Stellar network passphrases

use lazy_static::lazy_static;
use sp_std::vec::Vec;

use crate::utils::sha256::{sha256, BinarySha256Hash};

/// A wrapper type for the passphrase of a Stellar network
///
/// The SHA-256 hash of this passphrase is used for signing transactions.
/// This makes sure that a signed transaction is only valid for
/// a network having the specified passphrase.
pub struct Network {
    passphrase: Vec<u8>,
    id: BinarySha256Hash,
}

impl Network {
    /// Construct a new `Network` for the given `passphrase`
    pub fn new(passphrase: &[u8]) -> Network {
        let id = sha256(passphrase);
        let passphrase = passphrase.to_vec();
        Network { passphrase, id }
    }

    /// Return the passphrase of the network
    pub fn get_passphrase(&self) -> &Vec<u8> {
        &self.passphrase
    }

    /// Return the SHA-256 hash of the passphrase
    ///
    /// This hash is used for signing transactions.
    pub fn get_id(&self) -> &BinarySha256Hash {
        &self.id
    }
}

lazy_static! {
    /// The `Network` for the standard test network passphrase
    ///
    /// This passphrase is `"Test SDF Network ; September 2015"`.
    pub static ref TEST_NETWORK: Network = Network::new(b"Test SDF Network ; September 2015");

    /// The `Network` for the standard public network passphrase
    ///
    /// This passphrase is `"Public Global Stellar Network ; September 2015"`.
    pub static ref PUBLIC_NETWORK: Network =
        Network::new(b"Public Global Stellar Network ; September 2015");
}
