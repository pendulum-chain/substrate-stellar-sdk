use crate::{
    types::{ManageBuyOfferOp, OperationBody},
    Asset, IntoAmount, Operation, Price, StellarSdkError,
};

impl Operation {
    pub fn new_manage_buy_offer<S: IntoAmount>(
        selling: Asset,
        buying: Asset,
        buy_amount: S,
        price: Price,
        offer_id: Option<i64>,
    ) -> Result<Operation, StellarSdkError> {
        Ok(Operation {
            source_account: None,
            body: OperationBody::ManageBuyOffer(ManageBuyOfferOp {
                selling,
                buying,
                buy_amount: buy_amount.into_stroop_amount(true)?,
                price,
                offer_id: offer_id.unwrap_or(0),
            }),
        })
    }
}
