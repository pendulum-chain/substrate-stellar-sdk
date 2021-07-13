use crate::{
    types::{ManageSellOfferOp, OperationBody},
    AsAmount, Asset, Error, IntoMuxedAccountId, Operation, Price,
};

impl Operation {
    pub fn new_manage_sell_offer<T: IntoMuxedAccountId, S: AsAmount>(
        source_account: Option<T>,
        selling: Asset,
        buying: Asset,
        amount: S,
        price: Price,
        offer_id: Option<i64>,
    ) -> Result<Operation, Error> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        Ok(Operation {
            source_account,
            body: OperationBody::ManageSellOffer(ManageSellOfferOp {
                selling,
                buying,
                amount: amount.as_stroop_amount(true)?,
                price,
                offer_id: offer_id.unwrap_or(0),
            }),
        })
    }
}
