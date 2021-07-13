use crate::{
    types::{AllowTrustOp, OperationBody},
    AssetCode, IntoAccountId, IntoMuxedAccountId, Operation, StellarSdkError, TrustLineFlags,
};

impl Operation {
    pub fn new_allow_trust<T: IntoAccountId, S: AsRef<[u8]>, U: IntoMuxedAccountId>(
        source_account: Option<U>,
        trustor: T,
        asset_code: S,
        authorize: Option<TrustLineFlags>,
    ) -> Result<Operation, StellarSdkError> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

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
            source_account,
            body: OperationBody::AllowTrust(AllowTrustOp {
                trustor: trustor.into_account_id()?,
                asset: AssetCode::new(asset_code)?,
                authorize,
            }),
        })
    }
}
