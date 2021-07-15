use crate::{
    types::{BeginSponsoringFutureReservesOp, OperationBody},
    IntoAccountId, Operation, StellarSdkError,
};

impl Operation {
    pub fn new_begin_sponsoring_future_reserves<T: IntoAccountId>(
        sponsored_account_id: T,
    ) -> Result<Operation, StellarSdkError> {
        Ok(Operation {
            source_account: None,
            body: OperationBody::BeginSponsoringFutureReserves(BeginSponsoringFutureReservesOp {
                sponsored_id: sponsored_account_id.into_account_id()?,
            }),
        })
    }
}
