use crate::{
    types::{CreateAccountOp, OperationBody},
    IntoAccountId, IntoAmount, Operation, StellarSdkError,
};

impl Operation {
    pub fn new_create_account<T: IntoAccountId, U: IntoAmount>(
        destination: T,
        starting_balance: U,
    ) -> Result<Operation, StellarSdkError> {
        Ok(Operation {
            source_account: None,
            body: OperationBody::CreateAccount(CreateAccountOp {
                destination: destination.into_account_id()?,
                starting_balance: starting_balance.into_stroop_amount(true)?,
            }),
        })
    }
}
