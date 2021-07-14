use crate::{types::OperationBody, IntoMuxedAccountId, Operation, StellarSdkError};

impl Operation {
    pub fn new_account_merge<T: IntoMuxedAccountId, S: IntoMuxedAccountId>(
        source_account: Option<T>,
        destination_account: S,
    ) -> Result<Operation, StellarSdkError> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        Ok(Operation {
            source_account,
            body: OperationBody::AccountMerge(destination_account.into_muxed_account_id()?),
        })
    }
}
