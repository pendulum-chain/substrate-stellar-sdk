use crate::{
    types::{ClaimClaimableBalanceOp, OperationBody},
    StellarSdkError, IntoClaimbleBalanceId, IntoMuxedAccountId, Operation,
};

impl Operation {
    pub fn new_claim_claimable_balance<T: IntoClaimbleBalanceId, S: IntoMuxedAccountId>(
        source_account: Option<S>,
        balance_id: T,
    ) -> Result<Operation, StellarSdkError> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        let balance_id = balance_id.into_claimable_balance_id()?;

        Ok(Operation {
            source_account,
            body: OperationBody::ClaimClaimableBalance(ClaimClaimableBalanceOp { balance_id }),
        })
    }
}
