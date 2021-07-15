use crate::{
    types::{CreatePassiveSellOfferOp, OperationBody},
    Asset, IntoAmount, Operation, Price, StellarSdkError,
};

impl Operation {
    pub fn new_create_passive_sell_offser<S: IntoAmount>(
        selling: Asset,
        buying: Asset,
        amount: S,
        price: Price,
    ) -> Result<Operation, StellarSdkError> {
        Ok(Operation {
            source_account: None,
            body: OperationBody::CreatePassiveSellOffer(CreatePassiveSellOfferOp {
                selling,
                buying,
                amount: amount.into_stroop_amount(false)?,
                price,
            }),
        })
    }
}
