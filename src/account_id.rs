use crate::{
    types::{MuxedAccountMed25519, Uint64},
    utils::key_encoding::{
        decode_stellar_key, encode_stellar_key, ED25519_PUBLIC_KEY_VERSION_BYTE,
        MED25519_PUBLIC_KEY_BYTE_LENGTH, MED25519_PUBLIC_KEY_VERSION_BYTE,
    },
    AccountId, Error, MuxedAccount, PublicKey, XdrCodec,
};
use core::convert::TryInto;

pub trait IntoAccountId: Sized {
    fn into_account_id(self) -> Result<AccountId, Error>;
}

impl IntoAccountId for AccountId {
    fn into_account_id(self) -> Result<AccountId, Error> {
        Ok(self.clone())
    }
}

impl<T: AsRef<[u8]>> IntoAccountId for T {
    fn into_account_id(self) -> Result<AccountId, Error> {
        Ok(AccountId::from_encoding(self)?)
    }
}

impl MuxedAccount {
    pub fn from_encoding<T: AsRef<[u8]>>(encoded_key: T) -> Result<Self, Error> {
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

impl From<AccountId> for MuxedAccount {
    fn from(account_id: AccountId) -> Self {
        MuxedAccount::KeyTypeEd25519(account_id.into_binary())
    }
}

// This can be both an account id or a muxed account id.
// For that reason it would be better to call it AsPossiblyMuxedAccountId
// but Stellar just calls this a MuxedAccount, too.
pub trait IntoMuxedAccountId: Sized {
    fn into_muxed_account_id(self) -> Result<MuxedAccount, Error>;
}

impl IntoMuxedAccountId for AccountId {
    fn into_muxed_account_id(self) -> Result<MuxedAccount, Error> {
        Ok(MuxedAccount::KeyTypeEd25519(self.as_binary().clone()))
    }
}

impl IntoMuxedAccountId for MuxedAccount {
    fn into_muxed_account_id(self) -> Result<MuxedAccount, Error> {
        Ok(self.clone())
    }
}

impl<T: AsRef<[u8]>> IntoMuxedAccountId for T {
    fn into_muxed_account_id(self) -> Result<MuxedAccount, Error> {
        MuxedAccount::from_encoding(self)
    }
}
