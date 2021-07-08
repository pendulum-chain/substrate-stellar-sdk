//! An SDK for Stellar that can be used in Substrate projects

#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
pub use alloc::string::String;
#[cfg(feature = "std")]
pub use std::string::String;

pub mod keypair;
pub mod network;
pub mod stellar;
mod utils;

pub const BASE_FEE_STROOPS: u32 = 100;

#[cfg(offchain)]
pub mod horizon;
#[cfg(offchain)]
pub mod horizon_types;

pub use stellar::{types, xdr_codec::XdrCodec};
