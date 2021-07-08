//! Trait for types that can be XDR encoded/decoded

use base64::{decode_config_slice, encode_config_slice};
use core::convert::{AsRef, TryInto};
use sp_std::{boxed::Box, vec::Vec};

use super::streams::{DecodeError, ReadStream, WriteStream};

/// The XDR decoder/encoder trait
///
/// A type that implements this trait can be encoded as XDR or decoded from XDR
pub trait XdrCodec: Sized {
    /// Encode this type as XDR
    ///
    /// The binary XDR is returned as a byte vector
    fn to_xdr(&self) -> Vec<u8> {
        let mut write_stream = WriteStream::new();
        self.to_xdr_buffered(&mut write_stream);
        write_stream.get_result()
    }

    /// Decode XDR provided as a reference to a byte vector
    ///
    /// This will return error if decoding was not successful
    fn from_xdr<T: AsRef<[u8]>>(input: T) -> Result<Self, DecodeError> {
        let mut read_stream = ReadStream::new(input);
        let value = Self::from_xdr_buffered(&mut read_stream)?;
        if read_stream.no_of_bytes_left_to_read() != 0 {
            return Err(DecodeError::TypeEndsTooEarly {
                remaining_no_of_bytes: read_stream.no_of_bytes_left_to_read(),
            });
        }

        Ok(value)
    }

    /// Encode this type as base64 encoded XDR
    ///
    /// This returns an ASCII string (as a byte vector) that is the base64 encoding
    /// of the XDR encoding of this type.
    fn to_base64_xdr(&self) -> Vec<u8> {
        let xdr = self.to_xdr();
        let mut base64_buffer = Vec::new();
        base64_buffer.resize(xdr.len() * 4 / 3 + 4, 0);
        let bytes_written = encode_config_slice(xdr, base64::STANDARD, &mut base64_buffer);
        base64_buffer.resize(bytes_written, 0);
        base64_buffer
    }

    /// Decode this type from base64 encoded XDR
    ///
    /// This takes a reference to an ASCII string (as a byte vector), decodes it as base64
    /// and then decodes the resulting binary array as XDR.
    fn from_base64_xdr<T: AsRef<[u8]>>(input: T) -> Result<Self, DecodeError> {
        let input = input.as_ref();
        let mut buf = Vec::new();
        buf.resize(input.len() * 4 / 3 + 4, 0);

        match decode_config_slice(input, base64::STANDARD, &mut buf) {
            Ok(bytes_written) => {
                buf.resize(bytes_written, 0);
                Self::from_xdr(buf)
            }
            Err(_) => Err(DecodeError::InvalidBase64),
        }
    }

    /// Encode the XDR to a write stream
    ///
    /// This is the basic implementation of the XDR encoder of this type. The methods
    /// `to_xdr` and `to_base64_xdr` call this function to do the heavy lifting.
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream);

    /// Decode the XDR from a read stream
    ///
    /// This is the basic implementation of the XDR decoder of this type. The methods
    /// `from_xdr` and `from_base64_xdr` call this function to do the heavy lifting.
    fn from_xdr_buffered<T: AsRef<[u8]>>(
        read_stream: &mut ReadStream<T>,
    ) -> Result<Self, DecodeError>;
}

/// Implementation of the XDR decoder/encoder for `u64`
impl XdrCodec for u64 {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) {
        write_stream.write_next_u64(*self);
    }

    fn from_xdr_buffered<T: AsRef<[u8]>>(
        read_stream: &mut ReadStream<T>,
    ) -> Result<Self, DecodeError> {
        read_stream.read_next_u64()
    }
}

/// Implementation of the XDR decoder/encoder for `i64`
impl XdrCodec for i64 {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) {
        write_stream.write_next_i64(*self);
    }

    fn from_xdr_buffered<T: AsRef<[u8]>>(
        read_stream: &mut ReadStream<T>,
    ) -> Result<Self, DecodeError> {
        read_stream.read_next_i64()
    }
}

/// Implementation of the XDR decoder/encoder for `u32`
impl XdrCodec for u32 {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) {
        write_stream.write_next_u32(*self);
    }

    fn from_xdr_buffered<T: AsRef<[u8]>>(
        read_stream: &mut ReadStream<T>,
    ) -> Result<Self, DecodeError> {
        read_stream.read_next_u32()
    }
}

/// Implementation of the XDR decoder/encoder for `i32`
impl XdrCodec for i32 {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) {
        write_stream.write_next_i32(*self);
    }

    fn from_xdr_buffered<T: AsRef<[u8]>>(
        read_stream: &mut ReadStream<T>,
    ) -> Result<Self, DecodeError> {
        read_stream.read_next_i32()
    }
}

/// Implementation of the XDR decoder/encoder for `bool`
impl XdrCodec for bool {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) {
        write_stream.write_next_i32(if *self { 1 } else { 0 });
    }

    fn from_xdr_buffered<T: AsRef<[u8]>>(
        read_stream: &mut ReadStream<T>,
    ) -> Result<Self, DecodeError> {
        let parsed_int = read_stream.read_next_i32()?;
        match parsed_int {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(DecodeError::InvalidBoolean {
                found_integer: parsed_int,
                at_position: read_stream.get_position(),
            }),
        }
    }
}

/// Implementation of the XDR decoder/encoder for a fixed size array
///
/// This requires that the inner type already implements `XdrCodec`
impl<T: XdrCodec, const N: usize> XdrCodec for [T; N] {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) {
        for item in self.iter() {
            item.to_xdr_buffered(write_stream);
        }
    }

    fn from_xdr_buffered<R: AsRef<[u8]>>(
        read_stream: &mut ReadStream<R>,
    ) -> Result<Self, DecodeError> {
        let mut result = Vec::<T>::with_capacity(N);
        for _ in 0..N {
            result.push(T::from_xdr_buffered(read_stream)?)
        }
        result.try_into().map_err(|_| unreachable!())
    }
}

/// Implementation of the XDR decoder/encoder for fixed length binary data
impl<const N: usize> XdrCodec for [u8; N] {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) {
        write_stream.write_next_binary_data(self);
    }

    fn from_xdr_buffered<T: AsRef<[u8]>>(
        read_stream: &mut ReadStream<T>,
    ) -> Result<Self, DecodeError> {
        let value = read_stream.read_next_binary_data(N)?;
        value.try_into().map_err(|_| unreachable!())
    }
}

/// Implementation of the XDR decoder/encoder for an `Option`.
///
/// This requires that the inner type already implements `XdrCodec`
impl<T: XdrCodec> XdrCodec for Option<T> {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) {
        match self {
            None => write_stream.write_next_u32(0),
            Some(value) => {
                write_stream.write_next_u32(1);
                value.to_xdr_buffered(write_stream);
            }
        }
    }

    fn from_xdr_buffered<R: AsRef<[u8]>>(
        read_stream: &mut ReadStream<R>,
    ) -> Result<Self, DecodeError> {
        match read_stream.read_next_u32()? {
            0 => Ok(None),
            1 => T::from_xdr_buffered(read_stream).map(|ok| Some(ok)),
            code => Err(DecodeError::InvalidOptional {
                at_position: read_stream.get_position(),
                has_code: code,
            }),
        }
    }
}

/// Implementation of the XDR decoder/encoder for an `Box`.
///
/// This requires that the inner type already implements `XdrCodec`
impl<T: XdrCodec> XdrCodec for Box<T> {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) {
        self.as_ref().to_xdr_buffered(write_stream)
    }

    fn from_xdr_buffered<R: AsRef<[u8]>>(
        read_stream: &mut ReadStream<R>,
    ) -> Result<Self, DecodeError> {
        Ok(Box::new(T::from_xdr_buffered(read_stream)?))
    }
}
