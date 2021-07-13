use crate::{AsBinary, ClaimableBalanceId, Error, XdrCodec};

pub trait IntoClaimbleBalanceId {
    fn into_claimable_balance_id(self) -> Result<ClaimableBalanceId, Error>;
}

impl IntoClaimbleBalanceId for ClaimableBalanceId {
    fn into_claimable_balance_id(self) -> Result<ClaimableBalanceId, Error> {
        Ok(self)
    }
}

impl<T: AsRef<[u8]>> IntoClaimbleBalanceId for AsBinary<T> {
    fn into_claimable_balance_id(self) -> Result<ClaimableBalanceId, Error> {
        let balance_id: [u8; 4 + 32] = self.as_binary()?;

        ClaimableBalanceId::from_xdr(balance_id).map_err(|_| Error::InvalidBalanceId)
    }
}
