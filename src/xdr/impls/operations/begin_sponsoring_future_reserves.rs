use crate::{
    types::{BeginSponsoringFutureReservesOp, OperationBody},
    StellarSdkError, IntoAccountId, IntoMuxedAccountId, Operation,
};

impl Operation {
    pub fn new_begin_sponsoring_future_reserves<T: IntoAccountId, S: IntoMuxedAccountId>(
        source_account: Option<S>,
        sponsored_account_id: T,
    ) -> Result<Operation, StellarSdkError> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        Ok(Operation {
            source_account,
            body: OperationBody::BeginSponsoringFutureReserves(BeginSponsoringFutureReservesOp {
                sponsored_id: sponsored_account_id.into_account_id()?,
            }),
        })
    }
}
