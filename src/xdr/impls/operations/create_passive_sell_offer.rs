use crate::{
    types::{CreatePassiveSellOfferOp, OperationBody},
    Asset, IntoAmount, IntoMuxedAccountId, Operation, Price, StellarSdkError,
};

impl Operation {
    pub fn new_create_passive_sell_offser<T: IntoMuxedAccountId, S: IntoAmount>(
        source_account: Option<T>,
        selling: Asset,
        buying: Asset,
        amount: S,
        price: Price,
    ) -> Result<Operation, StellarSdkError> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        Ok(Operation {
            source_account,
            body: OperationBody::CreatePassiveSellOffer(CreatePassiveSellOfferOp {
                selling,
                buying,
                amount: amount.into_stroop_amount(false)?,
                price,
            }),
        })
    }
}
