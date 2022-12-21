use crate::{
    types::{ChangeTrustAsset, ChangeTrustOp, OperationBody},
    IntoAmount, Operation, StellarSdkError,
};

impl Operation {
    pub fn new_change_trust(line: ChangeTrustAsset) -> Result<Operation, StellarSdkError> {
        Ok(Operation {
            source_account: None,
            body: OperationBody::ChangeTrust(ChangeTrustOp { line, limit: i64::MAX }),
        })
    }

    pub fn new_change_trust_with_limit<T: IntoAmount>(
        line: ChangeTrustAsset,
        limit: T,
    ) -> Result<Operation, StellarSdkError> {
        Ok(Operation {
            source_account: None,
            body: OperationBody::ChangeTrust(ChangeTrustOp { line, limit: limit.into_stroop_amount(true)? }),
        })
    }
}
