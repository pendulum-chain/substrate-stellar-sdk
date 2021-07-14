use crate::{
    types::{BumpSequenceOp, OperationBody},
    IntoMuxedAccountId, Operation, StellarSdkError,
};

impl Operation {
    pub fn new_bump_sequence<T: IntoMuxedAccountId>(
        source_account: Option<T>,
        bump_to: i64,
    ) -> Result<Operation, StellarSdkError> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        Ok(Operation {
            source_account,
            body: OperationBody::BumpSequence(BumpSequenceOp { bump_to }),
        })
    }
}
