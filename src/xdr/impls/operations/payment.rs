use crate::{
    types::{OperationBody, PaymentOp},
    Asset, IntoAmount, IntoMuxedAccountId, Operation, StellarSdkError,
};

impl Operation {
    pub fn new_payment<T: IntoMuxedAccountId, S: IntoMuxedAccountId, U: IntoAmount>(
        source_account: Option<T>,
        destination: S,
        asset: Asset,
        amount: U,
    ) -> Result<Operation, StellarSdkError> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        Ok(Operation {
            source_account,
            body: OperationBody::Payment(PaymentOp {
                destination: destination.into_muxed_account_id()?,
                asset,
                amount: amount.into_stroop_amount(false)?,
            }),
        })
    }
}
