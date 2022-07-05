use crate::{types::OperationBody, Operation, StellarSdkError};

impl Operation {
    pub fn new_inflation() -> Result<Operation, StellarSdkError> {
        Ok(Operation {
            source_account: None,
            body: OperationBody::Inflation,
        })
    }
}
