//! An SDK for Stellar that can be used in Substrate projects

#![cfg_attr(not(test), no_std)]

pub mod keypair;
pub mod network;
pub mod transaction;
mod utils;

pub use substrate_stellar_xdr::{xdr, xdr_codec::XdrCodec};
