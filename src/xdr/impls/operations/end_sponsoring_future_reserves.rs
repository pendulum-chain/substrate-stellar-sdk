use crate::{types::OperationBody, Operation, StellarSdkError};

impl Operation {
    pub fn new_end_sponsoring_future_reserves() -> Result<Operation, StellarSdkError> {
        Ok(Operation {
            source_account: None,
            body: OperationBody::EndSponsoringFutureReserves,
        })
    }
}
