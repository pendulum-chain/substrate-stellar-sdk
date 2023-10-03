/// A trait used to convert any Stellar specific type `T` as a String
/// This also immediately converts the standard Error to a user-defined Error `E`
/// Helpful for functions that will accept:
///  * the Stellar type itself;
///  * encoded &str version of the Stellar type;
///  * a `Vec<u8>` version of th Stellar type
pub trait StellarTypeToString<T, E: From<std::str::Utf8Error>> {
    fn as_encoded_string(&self) -> Result<String, E>;
}