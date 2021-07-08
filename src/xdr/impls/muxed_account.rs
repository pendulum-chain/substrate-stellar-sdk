use crate::{
    types::{MuxedAccount, MuxedAccountMed25519},
    AsPublicKey, Error, PublicKey,
};

impl MuxedAccount {
    pub fn from_account_id<T: AsPublicKey>(account_id: T) -> Result<Self, Error> {
        account_id.as_public_key().map(|account_id| {
            let account_id = match account_id {
                PublicKey::PublicKeyTypeEd25519(account_id) => account_id,
            };
            MuxedAccount::KeyTypeEd25519(account_id)
        })
    }

    pub fn from_muxed_account_id<T: AsPublicKey>(
        account_id: T,
        sub_account_id: u64,
    ) -> Result<Self, Error> {
        account_id.as_public_key().map(|account_id| {
            let account_id = match account_id {
                PublicKey::PublicKeyTypeEd25519(account_id) => account_id,
            };
            MuxedAccount::KeyTypeMuxedEd25519(MuxedAccountMed25519 {
                id: sub_account_id,
                ed25519: account_id,
            })
        })
    }
}
