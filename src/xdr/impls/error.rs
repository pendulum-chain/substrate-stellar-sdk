use crate::{lib::String, types::Error};

impl<E: From<sp_std::str::Utf8Error>> crate::StellarTypeToString<Self, E> for Error {
    fn as_encoded_string(&self) -> Result<String, E> {
        let msg = self.msg.get_vec();
        let msg = sp_std::str::from_utf8(msg).map_err(E::from)?.to_string();
        Ok(format!("Error{{ code:{:?} message:{msg} }}", self.code))
    }
}
