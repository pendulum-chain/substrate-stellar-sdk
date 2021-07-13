use crate::{
    types::{ChangeTrustOp, OperationBody},
    Asset, IntoAmount, IntoMuxedAccountId, Operation, StellarSdkError,
};

impl Operation {
    pub fn new_change_trust<T: IntoAmount, S: IntoMuxedAccountId>(
        source_account: Option<S>,
        line: Asset,
        limit: Option<T>,
    ) -> Result<Operation, StellarSdkError> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        let limit_stroops = match limit {
            Some(limit) => limit.into_stroop_amount(true)?,
            None => i64::MAX,
        };

        Ok(Operation {
            source_account,
            body: OperationBody::ChangeTrust(ChangeTrustOp {
                line,
                limit: limit_stroops,
            }),
        })
    }
}
