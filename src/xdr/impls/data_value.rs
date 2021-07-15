use crate::{compound_types::LimitedVarOpaque, DataValue, StellarSdkError};
use sp_std::vec::Vec;

pub trait IntoDataValue {
    fn into_data_value(self) -> Result<DataValue, StellarSdkError>;
}

impl IntoDataValue for DataValue {
    fn into_data_value(self) -> Result<DataValue, StellarSdkError> {
        Ok(self)
    }
}

impl<T: AsRef<[u8]>> IntoDataValue for T {
    fn into_data_value(self) -> Result<DataValue, StellarSdkError> {
        let value = self.as_ref();
        LimitedVarOpaque::new(Vec::from(value))
    }
}
