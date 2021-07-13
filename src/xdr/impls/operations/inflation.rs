use crate::{types::OperationBody, Error, IntoMuxedAccountId, Operation};

impl Operation {
    pub fn new_inflation<T: IntoMuxedAccountId>(
        source_account: Option<T>,
    ) -> Result<Operation, Error> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        Ok(Operation {
            source_account,
            body: OperationBody::Inflation,
        })
    }
}
