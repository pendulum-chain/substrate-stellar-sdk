use sp_std::vec::Vec;

use crate::{
    compound_types::LimitedVarArray,
    types::{CreateClaimableBalanceOp, OperationBody},
    AsAmount, Asset, Claimant, Error, IntoMuxedAccountId, Operation,
};

impl Operation {
    pub fn new_create_claimable_balance<T: IntoMuxedAccountId, S: AsAmount>(
        source_account: Option<T>,
        asset: Asset,
        amount: S,
        claimants: Vec<Claimant>,
    ) -> Result<Operation, Error> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        if claimants.is_empty() {
            return Err(Error::EmptyClaimants);
        }

        Ok(Operation {
            source_account,
            body: OperationBody::CreateClaimableBalance(CreateClaimableBalanceOp {
                asset,
                amount: amount.as_stroop_amount(false)?,
                claimants: LimitedVarArray::new(claimants)?,
            }),
        })
    }
}
