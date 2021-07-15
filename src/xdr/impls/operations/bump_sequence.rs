use crate::{
    types::{BumpSequenceOp, OperationBody},
    Operation, StellarSdkError,
};

impl Operation {
    pub fn new_bump_sequence(bump_to: i64) -> Result<Operation, StellarSdkError> {
        Ok(Operation {
            source_account: None,
            body: OperationBody::BumpSequence(BumpSequenceOp { bump_to }),
        })
    }
}
