use core::convert::AsRef;

use crate::{
    compound_types::LimitedString,
    types::{ManageDataOp, OperationBody},
    IntoDataValue, Operation, StellarSdkError,
};

impl Operation {
    pub fn new_manage_data_put<T: AsRef<[u8]>, S: IntoDataValue>(
        data_name: T,
        data_value: S,
    ) -> Result<Operation, StellarSdkError> {
        let data_name = data_name.as_ref().to_vec();

        Ok(Operation {
            source_account: None,
            body: OperationBody::ManageData(ManageDataOp {
                data_name: LimitedString::new(data_name)?,
                data_value: Some(data_value.into_data_value()?),
            }),
        })
    }

    pub fn new_manage_data_delete<T: AsRef<[u8]>>(
        data_name: T,
    ) -> Result<Operation, StellarSdkError> {
        let data_name = data_name.as_ref().to_vec();

        Ok(Operation {
            source_account: None,
            body: OperationBody::ManageData(ManageDataOp {
                data_name: LimitedString::new(data_name)?,
                data_value: None,
            }),
        })
    }
}
