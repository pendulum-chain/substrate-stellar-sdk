use lazy_static::lazy_static;
use sp_std::vec::Vec;

use super::sha256::{sha256, BinarySha256Hash};

pub struct Network {
    passphrase: Vec<u8>,
    id: BinarySha256Hash,
}

impl Network {
    pub fn new(passphrase: &[u8]) -> Network {
        let id = sha256(passphrase);
        let passphrase = passphrase.to_vec();
        Network { passphrase, id }
    }

    pub fn get_passphrase(&self) -> &Vec<u8> {
        &self.passphrase
    }

    pub fn get_id(&self) -> &BinarySha256Hash {
        &self.id
    }
}

lazy_static! {
    pub static ref TEST_NETWORK: Network = Network::new(b"Test SDF Network ; September 2015");
    pub static ref PUBLIC_NETWORK: Network =
        Network::new(b"Public Global Stellar Network ; September 2015");
}
