use sp_std::vec::Vec;

use crate::{
    compound_types::LimitedVarArray,
    types::{OperationBody, PathPaymentStrictReceiveOp},
    Asset, IntoAmount, IntoMuxedAccountId, Operation, StellarSdkError,
};

impl Operation {
    pub fn new_path_payment_strict_receive<
        S: IntoAmount,
        U: IntoAmount,
        V: IntoMuxedAccountId,
    >(
        send_asset: Asset,
        send_max: S,
        destination: V,
        dest_asset: Asset,
        dest_amount: U,
        path: Option<Vec<Asset>>,
    ) -> Result<Operation, StellarSdkError> {
        let path = match path {
            Some(path) => LimitedVarArray::new(path)?,
            None => LimitedVarArray::new_empty(),
        };

        Ok(Operation {
            source_account: None,
            body: OperationBody::PathPaymentStrictReceive(PathPaymentStrictReceiveOp {
                send_asset,
                send_max: send_max.into_stroop_amount(false)?,
                destination: destination.into_muxed_account_id()?,
                dest_asset,
                dest_amount: dest_amount.into_stroop_amount(false)?,
                path,
            }),
        })
    }
}
