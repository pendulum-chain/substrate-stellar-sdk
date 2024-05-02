use crate::{
    lib::{String, ToString},
    AsBinary, ClaimableBalanceId, StellarSdkError, XdrCodec,
};

pub trait IntoClaimbleBalanceId {
    fn into_claimable_balance_id(self) -> Result<ClaimableBalanceId, StellarSdkError>;
}

impl IntoClaimbleBalanceId for ClaimableBalanceId {
    fn into_claimable_balance_id(self) -> Result<ClaimableBalanceId, StellarSdkError> {
        Ok(self)
    }
}

impl<T: AsRef<[u8]>> IntoClaimbleBalanceId for AsBinary<T> {
    fn into_claimable_balance_id(self) -> Result<ClaimableBalanceId, StellarSdkError> {
        let balance_id: [u8; 4 + 32] = self.as_binary()?;

        ClaimableBalanceId::from_xdr(balance_id).map_err(|_| StellarSdkError::InvalidBalanceId)
    }
}

impl<E: From<sp_std::str::Utf8Error>> crate::StellarTypeToString<Self, E> for ClaimableBalanceId {
    fn as_encoded_string(&self) -> Result<String, E> {
        let xdr = self.to_xdr();
        Ok(hex::encode(xdr))
    }
}

impl<E: From<sp_std::str::Utf8Error>> crate::StellarTypeToString<ClaimableBalanceId, E> for &str {
    fn as_encoded_string(&self) -> Result<String, E> {
        Ok(self.to_string())
    }
}
