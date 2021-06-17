use base64::{decode_config_slice, encode_config_slice, DecodeError};
use core::convert::AsRef;
use sp_std::vec::Vec;

pub fn encode<T: AsRef<[u8]>>(binary: T) -> Vec<u8> {
    let binary = binary.as_ref();
    let mut buf = Vec::new();
    buf.resize(binary.len() * 4 / 3 + 4, 0);

    let bytes_written = encode_config_slice(binary, base64::STANDARD, &mut buf);

    buf.resize(bytes_written, 0);
    buf
}

pub fn decode<T: AsRef<[u8]>>(binary: T) -> Result<Vec<u8>, DecodeError> {
    let binary = binary.as_ref();
    let mut buf = Vec::new();
    buf.resize(binary.len() * 4 / 3 + 4, 0);

    let bytes_written = decode_config_slice(binary, base64::STANDARD, &mut buf)?;

    buf.resize(bytes_written, 0);
    Ok(buf)
}
