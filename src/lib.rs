//! An SDK for Stellar that can be used in Substrate projects

#![cfg_attr(not(any(test, feature = "std")), no_std)]
// #![warn(missing_docs)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod lib {
    #[cfg(not(feature = "std"))]
    pub use alloc::string::{String, ToString, FromUtf8Error};
    #[cfg(feature = "std")]
    pub use std::string::{String, ToString, FromUtf8Error};
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

pub use xdr::{
    compound_types,
    impls::{
        account_id::IntoAccountId, claimable_balance_id::IntoClaimbleBalanceId, data_value::IntoDataValue,
        hash::IntoHash, muxed_account::IntoMuxedAccountId, time_bounds::*,
    },
    streams::{ReadStream, WriteStream},
    types::{
        self, AccountId, Asset, AssetCode, ClaimPredicate, ClaimableBalanceId, Claimant, Curve25519Secret, DataValue,
        FeeBumpTransaction, Hash, LedgerKey, Memo, MuxedAccount, Operation, Price, PublicKey, Signer, SignerKey,
        TimeBounds, Transaction, TransactionEnvelope, TrustLineFlags,
    },
    xdr_codec::XdrCodec,
};

#[cfg(feature = "all-types")]
pub use xdr::impls::transaction_set_type::TransactionSetType;

pub use utils::std::StellarTypeToString;

pub use amount::*;
pub use binary::*;
pub use public_key::*;
pub use secret_key::*;
