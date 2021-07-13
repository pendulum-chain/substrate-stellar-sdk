use crate::{
    types::{CreatePassiveSellOfferOp, OperationBody},
    AsAmount, Asset, Error, IntoMuxedAccountId, Operation, Price,
};

impl Operation {
    pub fn new_create_passive_sell_offser<T: IntoMuxedAccountId, S: AsAmount>(
        source_account: Option<T>,
        selling: Asset,
        buying: Asset,
        amount: S,
        price: Price,
    ) -> Result<Operation, Error> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        Ok(Operation {
            source_account,
            body: OperationBody::CreatePassiveSellOffer(CreatePassiveSellOfferOp {
                selling,
                buying,
                amount: amount.as_stroop_amount(false)?,
                price,
            }),
        })
    }
}
