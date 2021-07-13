use crate::{
    types::{OperationBody, SetTrustLineFlagsOp},
    Asset, IntoAccountId, IntoMuxedAccountId, Operation, StellarSdkError, TrustLineFlags,
};

impl Operation {
    pub fn new_set_trustline_flags<T: IntoAccountId, S: IntoMuxedAccountId>(
        source_account: Option<S>,
        trustor: T,
        asset: Asset,
        clear_flags: Vec<TrustLineFlags>,
        set_flags: Vec<TrustLineFlags>,
    ) -> Result<Operation, StellarSdkError> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        let clear_flags = clear_flags.iter().fold(0, |a, b| a | (*b as u32));
        let set_flags = set_flags.iter().fold(0, |a, b| a | (*b as u32));

        Ok(Operation {
            source_account,
            body: OperationBody::SetTrustLineFlags(SetTrustLineFlagsOp {
                trustor: trustor.into_account_id()?,
                asset,
                clear_flags,
                set_flags,
            }),
        })
    }
}
