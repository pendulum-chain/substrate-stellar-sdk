use crate::{
    types::{OperationBody, PaymentOp},
    AsAmount, Asset, Error, IntoMuxedAccountId, Operation,
};

impl Operation {
    pub fn new_payment<T: IntoMuxedAccountId, S: IntoMuxedAccountId, U: AsAmount>(
        source_account: Option<T>,
        destination: S,
        asset: Asset,
        amount: U,
    ) -> Result<Operation, Error> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        Ok(Operation {
            source_account,
            body: OperationBody::Payment(PaymentOp {
                destination: destination.into_muxed_account_id()?,
                asset,
                amount: amount.as_stroop_amount(false)?,
            }),
        })
    }
}
