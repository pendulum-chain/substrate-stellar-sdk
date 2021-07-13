use core::convert::AsRef;

use crate::{
    compound_types::LimitedString,
    types::{ManageDataOp, OperationBody},
    IntoDataValue, IntoMuxedAccountId, Operation, StellarSdkError,
};

impl Operation {
    pub fn new_manage_data<T: AsRef<[u8]>, S: IntoDataValue, U: IntoMuxedAccountId>(
        source_account: Option<U>,
        data_name: T,
        data_value: Option<S>,
    ) -> Result<Operation, StellarSdkError> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        let data_name = data_name.as_ref().to_vec();
        let data_value = match data_value {
            Some(data_value) => Some(data_value.into_data_value()?),
            None => None,
        };

        Ok(Operation {
            source_account,
            body: OperationBody::ManageData(ManageDataOp {
                data_name: LimitedString::new(data_name)?,
                data_value,
            }),
        })
    }
}
