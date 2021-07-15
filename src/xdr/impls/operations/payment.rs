use crate::{
    types::{OperationBody, PaymentOp},
    Asset, IntoAmount, IntoMuxedAccountId, Operation, StellarSdkError,
};

impl Operation {
    pub fn new_payment<S: IntoMuxedAccountId, U: IntoAmount>(
        destination: S,
        asset: Asset,
        amount: U,
    ) -> Result<Operation, StellarSdkError> {
        Ok(Operation {
            source_account: None,
            body: OperationBody::Payment(PaymentOp {
                destination: destination.into_muxed_account_id()?,
                asset,
                amount: amount.into_stroop_amount(false)?,
            }),
        })
    }
}
