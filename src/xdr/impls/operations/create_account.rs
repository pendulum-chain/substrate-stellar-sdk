use crate::{
    types::{CreateAccountOp, OperationBody},
    AsAmount, Error, IntoAccountId, IntoMuxedAccountId, Operation,
};

impl Operation {
    pub fn new_create_account<T: IntoAccountId, S: IntoMuxedAccountId, U: AsAmount>(
        source_account: Option<S>,
        destination: T,
        starting_balance: U,
    ) -> Result<Operation, Error> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        Ok(Operation {
            source_account,
            body: OperationBody::CreateAccount(CreateAccountOp {
                destination: destination.into_account_id()?,
                starting_balance: starting_balance.as_stroop_amount(true)?,
            }),
        })
    }
}
