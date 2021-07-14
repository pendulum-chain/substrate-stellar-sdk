use crate::{AccountId, MuxedAccount, StellarSdkError};
use sp_std::vec::Vec;

pub trait IntoAccountId: Sized {
    fn into_account_id(self) -> Result<AccountId, StellarSdkError>;

    // this function is for efficiency, if the only reason to convert
    // self into an account id is to get the encoding in the next step
    // in that case it would be a waste to first decode and then encode again
    fn into_encoding(self) -> Vec<u8>;
}

impl IntoAccountId for AccountId {
    fn into_account_id(self) -> Result<AccountId, StellarSdkError> {
        Ok(self)
    }

    fn into_encoding(self) -> Vec<u8> {
        self.to_encoding()
    }
}

impl<T: AsRef<[u8]>> IntoAccountId for T {
    fn into_account_id(self) -> Result<AccountId, StellarSdkError> {
        Ok(AccountId::from_encoding(self)?)
    }

    fn into_encoding(self) -> Vec<u8> {
        self.as_ref().to_vec()
    }
}

impl From<AccountId> for MuxedAccount {
    fn from(account_id: AccountId) -> Self {
        MuxedAccount::KeyTypeEd25519(account_id.into_binary())
    }
}
