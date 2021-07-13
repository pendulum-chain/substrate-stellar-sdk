use crate::{AccountId, MuxedAccount, StellarSdkError};

pub trait IntoAccountId: Sized {
    fn into_account_id(self) -> Result<AccountId, StellarSdkError>;
}

impl IntoAccountId for AccountId {
    fn into_account_id(self) -> Result<AccountId, StellarSdkError> {
        Ok(self.clone())
    }
}

impl<T: AsRef<[u8]>> IntoAccountId for T {
    fn into_account_id(self) -> Result<AccountId, StellarSdkError> {
        Ok(AccountId::from_encoding(self)?)
    }
}

impl From<AccountId> for MuxedAccount {
    fn from(account_id: AccountId) -> Self {
        MuxedAccount::KeyTypeEd25519(account_id.into_binary())
    }
}
