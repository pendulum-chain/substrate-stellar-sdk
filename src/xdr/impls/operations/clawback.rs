use crate::{
    amount::AsAmount,
    types::{ClawbackOp, OperationBody},
    Asset, Error, IntoAccountId, IntoMuxedAccountId, Operation,
};

impl Operation {
    pub fn new_clawback<T: AsAmount, S: IntoMuxedAccountId, U: IntoAccountId>(
        source_account: Option<S>,
        asset: Asset,
        amount: T,
        from: U,
    ) -> Result<Operation, Error> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        Ok(Operation {
            source_account,
            body: OperationBody::Clawback(ClawbackOp {
                asset,
                from: from.into_account_id()?.into(),
                amount: amount.as_stroop_amount(false)?,
            }),
        })
    }
}
