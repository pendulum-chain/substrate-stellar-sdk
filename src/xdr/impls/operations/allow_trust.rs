use crate::{
    types::{AllowTrustOp, OperationBody},
    AsPublicKey, AssetCode, Error, MuxedAccount, Operation,
};

impl Operation {
    pub fn new_allow_trust<T: AsPublicKey, S: AsRef<[u8]>>(
        source_account: Option<MuxedAccount>,
        trustor: T,
        asset: S,
        authorize: u32,
    ) -> Result<Operation, Error> {
        Ok(Operation {
            source_account,
            body: OperationBody::AllowTrust(AllowTrustOp {
                trustor: trustor.as_public_key()?,
                asset: AssetCode::new(asset)?,
                authorize,
            }),
        })
    }
}
