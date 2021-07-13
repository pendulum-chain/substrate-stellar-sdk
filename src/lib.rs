//! An SDK for Stellar that can be used in Substrate projects

#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod lib {
    #[cfg(all(not(feature = "std"), feature = "offchain"))]
    pub use alloc::string::String;
    #[cfg(feature = "std")]
    pub use std::string::String;
}

mod amount;
mod binary;
mod error;
pub mod network;
mod public_key;
mod secret_key;
mod utils;
mod xdr;

pub use error::StellarSdkError;

pub const BASE_FEE_STROOPS: u32 = 100;

#[cfg(feature = "offchain")]
pub mod horizon;
#[cfg(feature = "offchain")]
pub mod horizon_types;

pub use xdr::{
    compound_types,
    impls::{
        account_id::IntoAccountId, claimable_balance_id::IntoClaimbleBalanceId,
        data_value::IntoDataValue, hash::IntoHash, muxed_account::IntoMuxedAccountId,
    },
    types::{
        self, AccountId, Asset, AssetCode, ClaimPredicate, ClaimableBalanceId, Claimant,
        Curve25519Secret, DataValue, Hash, LedgerKey, Memo, MuxedAccount, Operation, Price,
        PublicKey, Signer, SignerKey, TimeBounds, Transaction, TrustLineFlags,
    },
    xdr_codec::XdrCodec,
};

pub use amount::*;
pub use binary::*;
pub use public_key::*;
pub use secret_key::*;
