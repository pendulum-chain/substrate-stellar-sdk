use sp_std::vec::Vec;

use crate::{
    compound_types::LimitedVarArray,
    types::{CreateClaimableBalanceOp, OperationBody},
    Asset, Claimant, IntoAmount, Operation, StellarSdkError,
};

impl Operation {
    pub fn new_create_claimable_balance<S: IntoAmount>(
        asset: Asset,
        amount: S,
        claimants: Vec<Claimant>,
    ) -> Result<Operation, StellarSdkError> {
        if claimants.is_empty() {
            return Err(StellarSdkError::EmptyClaimants)
        }

        Ok(Operation {
            source_account: None,
            body: OperationBody::CreateClaimableBalance(CreateClaimableBalanceOp {
                asset,
                amount: amount.into_stroop_amount(false)?,
                claimants: LimitedVarArray::new(claimants)?,
            }),
        })
    }
}
