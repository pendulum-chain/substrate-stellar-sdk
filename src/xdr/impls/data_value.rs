use crate::{compound_types::LimitedVarOpaque, DataValue, StellarSdkError};

pub trait IntoDataValue {
    fn into_data_value(self) -> Result<DataValue, StellarSdkError>;
}

impl IntoDataValue for DataValue {
    fn into_data_value(self) -> Result<DataValue, StellarSdkError> {
        Ok(self)
    }
}

impl IntoDataValue for &str {
    fn into_data_value(self) -> Result<DataValue, StellarSdkError> {
        self.as_bytes().to_vec().into_data_value()
    }
}

impl IntoDataValue for Vec<u8> {
    fn into_data_value(self) -> Result<DataValue, StellarSdkError> {
        LimitedVarOpaque::new(self)
    }
}
