use crate::{
    types::{AllowTrustOp, OperationBody},
    AssetCode, IntoAccountId, Operation, StellarSdkError, TrustLineFlags,
};

impl Operation {
    pub fn new_allow_trust<T: IntoAccountId, S: AsRef<[u8]>>(
        trustor: T,
        asset_code: S,
        authorize: Option<TrustLineFlags>,
    ) -> Result<Operation, StellarSdkError> {
        let authorize: u32 = match authorize {
            Some(authorize) => match authorize {
                TrustLineFlags::TrustlineClawbackEnabledFlag => {
                    return Err(StellarSdkError::InvalidAuthorizeFlag)
                }
                _ => authorize as u32,
            },
            None => 0,
        };

        Ok(Operation {
            source_account: None,
            body: OperationBody::AllowTrust(AllowTrustOp {
                trustor: trustor.into_account_id()?,
                asset: AssetCode::new(asset_code)?,
                authorize,
            }),
        })
    }
}
