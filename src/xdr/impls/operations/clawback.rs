use crate::{
    amount::IntoAmount,
    types::{ClawbackOp, OperationBody},
    Asset, IntoAccountId, IntoMuxedAccountId, Operation, StellarSdkError,
};

impl Operation {
    pub fn new_clawback<T: IntoAmount, S: IntoMuxedAccountId, U: IntoAccountId>(
        source_account: Option<S>,
        asset: Asset,
        amount: T,
        from: U,
    ) -> Result<Operation, StellarSdkError> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        Ok(Operation {
            source_account,
            body: OperationBody::Clawback(ClawbackOp {
                asset,
                from: from.into_account_id()?.into(),
                amount: amount.into_stroop_amount(false)?,
            }),
        })
    }
}
