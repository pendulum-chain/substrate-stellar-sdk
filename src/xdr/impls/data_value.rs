use crate::{compound_types::LimitedVarOpaque, DataValue, Error};

pub trait AsDataValue {
    fn as_data_value(self) -> Result<DataValue, Error>;
}

impl AsDataValue for DataValue {
    fn as_data_value(self) -> Result<DataValue, Error> {
        Ok(self)
    }
}

impl AsDataValue for &str {
    fn as_data_value(self) -> Result<DataValue, Error> {
        self.as_bytes().to_vec().as_data_value()
    }
}

impl AsDataValue for Vec<u8> {
    fn as_data_value(self) -> Result<DataValue, Error> {
        LimitedVarOpaque::new(self)
    }
}
