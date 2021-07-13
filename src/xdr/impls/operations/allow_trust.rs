use crate::{
    types::{AllowTrustOp, OperationBody},
    AssetCode, Error, IntoAccountId, IntoMuxedAccountId, Operation, TrustLineFlags,
};

impl Operation {
    pub fn new_allow_trust<T: IntoAccountId, S: AsRef<[u8]>, U: IntoMuxedAccountId>(
        source_account: Option<U>,
        trustor: T,
        asset_code: S,
        authorize: Option<TrustLineFlags>,
    ) -> Result<Operation, Error> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        let authorize: u32 = match authorize {
            Some(authorize) => match authorize {
                TrustLineFlags::TrustlineClawbackEnabledFlag => {
                    return Err(Error::InvalidAuthorizeFlag)
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
