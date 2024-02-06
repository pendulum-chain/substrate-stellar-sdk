use base64::DecodeError;
use hex::FromHexError;

#[cfg(feature = "offchain")]
use crate::horizon::FetchError;

#[derive(Debug, Clone, PartialEq)]
pub enum StellarSdkError {
    InvalidBase32Character {
        at_position: usize,
    },

    /// The encoding can be decoded but is not the canonical encoding of the underlying binary key
    InvalidStellarKeyEncoding,

    /// The encoding has an invalid length
    InvalidStellarKeyEncodingLength,

    /// The initial version byte is invalid for this `EncodableKey`
    InvalidStellarKeyEncodingVersion {
        expected_version: char,
        found_version: char,
    },

    /// The checksum in the encoding is invaliid
    InvalidStellarKeyChecksum {
        expected: u16,
        found: u16,
    },

    /// The signature has an invalid length
    InvalidSignatureLength {
        found_length: usize,
        expected_length: usize,
    },
    /// Verification for this public key failed
    PublicKeyCantVerify,

    /// The base64 encoding of the signature is invalid
    InvalidBase64Encoding(DecodeError),

    /// The transaction envelope already has the maximal number of signatures (20)
    TooManySignatures,

    AssetCodeTooLong,

    InvalidAssetCodeCharacter,

    ExceedsMaximumLength {
        requested_length: usize,
        allowed_length: i32,
    },

    InvalidHexEncoding(FromHexError),

    InvalidHashConversion,

    NotApproximableAsFraction,

    InvalidPrice,

    InvalidTrustLineLimit,

    InvalidAuthorizeFlag,

    InvalidAmountString,

    AmountOverflow,

    AmountNegative,

    AmountNonPositive,

    InvalidBinaryLength {
        found_length: usize,
        expected_length: usize,
    },

    InvalidBalanceId,

    EmptyClaimants,

    InvalidSignerWeight,

    CantWrapFeeBumpTransaction,

    #[cfg(feature = "offchain")]
    FetchError(FetchError),

    DecodeError(Vec<u8>), // String converted as Bytes of u8
}
