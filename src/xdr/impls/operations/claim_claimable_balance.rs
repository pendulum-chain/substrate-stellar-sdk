use crate::{
    types::{ClaimClaimableBalanceOp, OperationBody},
    IntoClaimbleBalanceId, Operation, StellarSdkError,
};

impl Operation {
    pub fn new_claim_claimable_balance<T: IntoClaimbleBalanceId>(balance_id: T) -> Result<Operation, StellarSdkError> {
        let balance_id = balance_id.into_claimable_balance_id()?;

        Ok(Operation {
            source_account: None,
            body: OperationBody::ClaimClaimableBalance(ClaimClaimableBalanceOp { balance_id }),
        })
    }
}
