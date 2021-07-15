use crate::{
    types::{ManageSellOfferOp, OperationBody},
    Asset, IntoAmount, Operation, Price, StellarSdkError,
};

impl Operation {
    pub fn new_manage_sell_offer<S: IntoAmount>(
        selling: Asset,
        buying: Asset,
        amount: S,
        price: Price,
        offer_id: Option<i64>,
    ) -> Result<Operation, StellarSdkError> {
        Ok(Operation {
            source_account: None,
            body: OperationBody::ManageSellOffer(ManageSellOfferOp {
                selling,
                buying,
                amount: amount.into_stroop_amount(true)?,
                price,
                offer_id: offer_id.unwrap_or(0),
            }),
        })
    }
}
