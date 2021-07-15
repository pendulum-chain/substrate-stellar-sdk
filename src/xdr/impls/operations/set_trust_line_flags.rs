use sp_std::vec::Vec;

use crate::{
    types::{OperationBody, SetTrustLineFlagsOp},
    Asset, IntoAccountId, Operation, StellarSdkError, TrustLineFlags,
};

impl Operation {
    pub fn new_set_trustline_flags<T: IntoAccountId>(
        trustor: T,
        asset: Asset,
        clear_flags: Vec<TrustLineFlags>,
        set_flags: Vec<TrustLineFlags>,
    ) -> Result<Operation, StellarSdkError> {
        let clear_flags = clear_flags.iter().fold(0, |a, b| a | (*b as u32));
        let set_flags = set_flags.iter().fold(0, |a, b| a | (*b as u32));

        Ok(Operation {
            source_account: None,
            body: OperationBody::SetTrustLineFlags(SetTrustLineFlagsOp {
                trustor: trustor.into_account_id()?,
                asset,
                clear_flags,
                set_flags,
            }),
        })
    }
}
