use crate::{
    types::{MuxedAccount, MuxedAccountMed25519, Uint64},
    utils::key_encoding::{
        decode_stellar_key, encode_stellar_key, ED25519_PUBLIC_KEY_VERSION_BYTE,
        MED25519_PUBLIC_KEY_BYTE_LENGTH, MED25519_PUBLIC_KEY_VERSION_BYTE,
    },
    AccountId, StellarSdkError, IntoPublicKey, PublicKey, XdrCodec,
};

use core::convert::TryInto;

impl MuxedAccount {
    pub fn from_account_id<T: IntoPublicKey>(account_id: T) -> Result<Self, StellarSdkError> {
        account_id.into_public_key().map(|account_id| {
            let account_id = match account_id {
                PublicKey::PublicKeyTypeEd25519(account_id) => account_id,
            };
            MuxedAccount::KeyTypeEd25519(account_id)
        })
    }

    pub fn from_muxed_account_id<T: IntoPublicKey>(
        account_id: T,
        sub_account_id: u64,
    ) -> Result<Self, StellarSdkError> {
        account_id.into_public_key().map(|account_id| {
            let account_id = match account_id {
                PublicKey::PublicKeyTypeEd25519(account_id) => account_id,
            };
            MuxedAccount::KeyTypeMuxedEd25519(MuxedAccountMed25519 {
                id: sub_account_id,
                ed25519: account_id,
            })
        })
    }

    pub fn from_encoding<T: AsRef<[u8]>>(encoded_key: T) -> Result<Self, StellarSdkError> {
        let encoded_key = encoded_key.as_ref();

        match decode_stellar_key::<_, MED25519_PUBLIC_KEY_BYTE_LENGTH>(
            encoded_key,
            MED25519_PUBLIC_KEY_VERSION_BYTE,
        ) {
            Ok(raw_bytes) => Ok(MuxedAccount::KeyTypeMuxedEd25519(MuxedAccountMed25519 {
                id: Uint64::from_xdr(&raw_bytes[MED25519_PUBLIC_KEY_BYTE_LENGTH - 8..]).unwrap(),
                ed25519: raw_bytes[..MED25519_PUBLIC_KEY_BYTE_LENGTH - 8]
                    .try_into()
                    .unwrap(),
            })),
            Err(_) => PublicKey::from_encoding(encoded_key)
                .map(|public_key| MuxedAccount::KeyTypeEd25519(public_key.into_binary())),
        }
    }

    /// Return the key encoding as an ASCII string (given as `Vec<u8>`)
    pub fn to_encoding(&self) -> Vec<u8> {
        match self {
            MuxedAccount::KeyTypeEd25519(raw_bytes) => {
                encode_stellar_key(raw_bytes, ED25519_PUBLIC_KEY_VERSION_BYTE)
            }
            MuxedAccount::KeyTypeMuxedEd25519(MuxedAccountMed25519 { id, ed25519 }) => {
                let mut raw_bytes = [0u8; MED25519_PUBLIC_KEY_BYTE_LENGTH];
                raw_bytes[..MED25519_PUBLIC_KEY_BYTE_LENGTH - 8].copy_from_slice(ed25519);
                raw_bytes[MED25519_PUBLIC_KEY_BYTE_LENGTH - 8..]
                    .copy_from_slice(id.to_xdr().as_slice());
                encode_stellar_key(&raw_bytes, MED25519_PUBLIC_KEY_VERSION_BYTE)
            }
            _ => unreachable!("Invalid muxed account type"),
        }
    }
}

// This can be both an account id or a muxed account id.
// For that reason it would be better to call it AsPossiblyMuxedAccountId
// but Stellar just calls this a MuxedAccount, too.
pub trait IntoMuxedAccountId: Sized {
    fn into_muxed_account_id(self) -> Result<MuxedAccount, StellarSdkError>;
}

impl IntoMuxedAccountId for AccountId {
    fn into_muxed_account_id(self) -> Result<MuxedAccount, StellarSdkError> {
        Ok(MuxedAccount::KeyTypeEd25519(self.as_binary().clone()))
    }
}

impl IntoMuxedAccountId for MuxedAccount {
    fn into_muxed_account_id(self) -> Result<MuxedAccount, StellarSdkError> {
        Ok(self.clone())
    }
}

impl<T: AsRef<[u8]>> IntoMuxedAccountId for T {
    fn into_muxed_account_id(self) -> Result<MuxedAccount, StellarSdkError> {
        MuxedAccount::from_encoding(self)
    }
}
