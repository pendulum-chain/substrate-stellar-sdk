//! An SDK for Stellar that can be used in Substrate projects

#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod lib {
    #[cfg(not(feature = "std"))]
    pub use alloc::string::String;
    #[cfg(feature = "std")]
    pub use std::string::String;
}

mod error;
pub mod keypair;
pub mod network;
mod utils;
mod xdr;

pub use error::Error;

pub const BASE_FEE_STROOPS: u32 = 100;

#[cfg(feature = "offchain")]
pub mod horizon;
#[cfg(feature = "offchain")]
pub mod horizon_types;

pub use keypair::AsPublicKey;

pub use xdr::{
    compound_types,
    impls::hash::AsHash,
    types::{
        self, AccountId, Asset, AssetCode, ClaimPredicate, Curve25519Secret, Hash, LedgerKey, Memo,
        MuxedAccount, Operation, Price, PublicKey, Signer, SignerKey, TimeBounds,
    },
    xdr_codec::XdrCodec,
};
