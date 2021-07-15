use crate::{
    amount::IntoAmount,
    types::{ClawbackOp, OperationBody},
    Asset, IntoAccountId, Operation, StellarSdkError,
};

impl Operation {
    pub fn new_clawback<T: IntoAmount, U: IntoAccountId>(
        asset: Asset,
        amount: T,
        from: U,
    ) -> Result<Operation, StellarSdkError> {
        Ok(Operation {
            source_account: None,
            body: OperationBody::Clawback(ClawbackOp {
                asset,
                from: from.into_account_id()?.into(),
                amount: amount.into_stroop_amount(false)?,
            }),
        })
    }
}
