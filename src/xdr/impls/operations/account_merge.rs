use crate::{types::OperationBody, IntoMuxedAccountId, Operation, StellarSdkError};

impl Operation {
    pub fn new_account_merge<S: IntoMuxedAccountId>(
        destination_account: S,
    ) -> Result<Operation, StellarSdkError> {
        Ok(Operation {
            source_account: None,
            body: OperationBody::AccountMerge(destination_account.into_muxed_account_id()?),
        })
    }
}
