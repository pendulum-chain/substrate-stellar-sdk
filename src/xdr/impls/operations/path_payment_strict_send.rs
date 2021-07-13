use sp_std::vec::Vec;

use crate::{
    compound_types::LimitedVarArray,
    types::{OperationBody, PathPaymentStrictSendOp},
    Asset, IntoAmount, IntoMuxedAccountId, Operation, StellarSdkError,
};

impl Operation {
    pub fn new_path_payment_strict_send<
        T: IntoMuxedAccountId,
        S: IntoAmount,
        U: IntoAmount,
        V: IntoMuxedAccountId,
    >(
        source_account: Option<T>,
        send_asset: Asset,
        send_amount: S,
        destination: V,
        dest_asset: Asset,
        dest_min: U,
        path: Option<Vec<Asset>>,
    ) -> Result<Operation, StellarSdkError> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        let path = match path {
            Some(path) => LimitedVarArray::new(path)?,
            None => LimitedVarArray::new_empty(),
        };

        Ok(Operation {
            source_account,
            body: OperationBody::PathPaymentStrictSend(PathPaymentStrictSendOp {
                send_asset,
                send_amount: send_amount.into_stroop_amount(false)?,
                destination: destination.into_muxed_account_id()?,
                dest_asset,
                dest_min: dest_min.into_stroop_amount(false)?,
                path,
            }),
        })
    }
}
