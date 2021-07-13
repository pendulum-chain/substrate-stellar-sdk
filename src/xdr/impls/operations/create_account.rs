use crate::{
    types::{CreateAccountOp, OperationBody},
    IntoAccountId, IntoAmount, IntoMuxedAccountId, Operation, StellarSdkError,
};

impl Operation {
    pub fn new_create_account<T: IntoAccountId, S: IntoMuxedAccountId, U: IntoAmount>(
        source_account: Option<S>,
        destination: T,
        starting_balance: U,
    ) -> Result<Operation, StellarSdkError> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        Ok(Operation {
            source_account,
            body: OperationBody::CreateAccount(CreateAccountOp {
                destination: destination.into_account_id()?,
                starting_balance: starting_balance.into_stroop_amount(true)?,
            }),
        })
    }
}
