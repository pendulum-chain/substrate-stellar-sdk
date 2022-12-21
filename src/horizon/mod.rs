use sp_std::vec::Vec;

mod json_response_types;

pub mod api_response_types;
pub mod fetch;
pub mod submit_transaction;

pub struct Horizon {
    base_url: Vec<u8>,
}

pub use fetch::FetchError;

impl Horizon {
    pub fn new(base_url: &str) -> Horizon {
        Horizon { base_url: base_url.as_bytes().to_vec() }
    }
}

const HTTP_HEADER_CLIENT_NAME: &str = "substrate-stellar-sdk";
const HTTP_HEADER_CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");
