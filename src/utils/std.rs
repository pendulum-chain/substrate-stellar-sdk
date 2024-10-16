use crate::lib::String;
use crate::XdrCodec;

/// A trait used to convert any Stellar specific type `T` as a String
/// This also immediately converts the standard Error to a user-defined Error `E`
/// Helpful for functions that will accept:
///  * the Stellar type itself;
///  * encoded &str version of the Stellar type;
///  * a `Vec<u8>` version of th Stellar type
pub trait StellarTypeToString<T, E: From<sp_std::str::Utf8Error>> {
    fn as_encoded_string(&self) -> Result<String, E>;
}

/// A trait used to convert a structure into Base64 String
pub trait StellarTypeToBase64String {
    /// returns a Base64 encoded String or a vec of bytes [xdr]
    fn as_base64_encoded_string(&self) -> String;
}

impl <T: XdrCodec> StellarTypeToBase64String for T {
    fn as_base64_encoded_string(&self) -> String {
        let xdr = self.to_base64_xdr();
        // safe to use `unwrap`, since `to_base64_xdr()` will always return a valid vec of
        String::from_utf8(xdr.clone()).unwrap()
    }
}