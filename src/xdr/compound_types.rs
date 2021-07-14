//! Generic types for encoding XDR variable length arrays and strings

use core::convert::AsRef;
use sp_std::{prelude::*, vec::Vec};

use super::streams::{DecodeError, ReadStream, WriteStream};
use super::xdr_codec::XdrCodec;
use crate::StellarSdkError;

/// Type for binary data whose length is not predefined but bounded by a constant
///
/// The const generic `N` specifies the maxmimum number of bytes a value of this
/// type is allowed to have.
#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LimitedVarOpaque<const N: i32>(Vec<u8>);

impl<const N: i32> LimitedVarOpaque<N> {
    /// Construct a new `LimitedVarOpaque` from a byte vector
    ///
    /// The length of the byte vector must not exceed `N`. Otherwise this function returns
    /// an error.
    pub fn new(vec: Vec<u8>) -> Result<Self, StellarSdkError> {
        match vec.len() > N as usize {
            true => Err(StellarSdkError::ExceedsMaximumLength {
                requested_length: vec.len(),
                allowed_length: N,
            }),
            false => Ok(LimitedVarOpaque(vec)),
        }
    }

    pub fn new_empty() -> Self {
        LimitedVarOpaque(vec![])
    }

    /// Returns a reference to the raw byte vector
    pub fn get_vec(&self) -> &Vec<u8> {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<const N: i32> XdrCodec for LimitedVarOpaque<N> {
    /// The XDR encoder implementation for `LimitedVarOpaque`
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) {
        write_stream.write_next_u32(self.0.len() as u32);
        write_stream.write_next_binary_data(&self.0[..]);
    }

    /// The XDR decoder implementation for `LimitedVarOpaque`
    fn from_xdr_buffered<R: AsRef<[u8]>>(
        read_stream: &mut ReadStream<R>,
    ) -> Result<Self, DecodeError> {
        let length = read_stream.read_next_u32()? as i32;
        match length > N {
            true => Err(DecodeError::VarOpaqueExceedsMaxLength {
                at_position: read_stream.get_position(),
                max_length: N,
                actual_length: length,
            }),
            false => Ok(
                LimitedVarOpaque::new(read_stream.read_next_binary_data(length as usize)?).unwrap(),
            ),
        }
    }
}

/// Type for binary data whose length is not predefined and not bounded
///
/// Actually an `UnlimitedVarOpaque` is limited: it must not have more than
/// `i32::MAX` bytes.
#[allow(dead_code)]
pub type UnlimitedVarOpaque = LimitedVarOpaque<{ i32::MAX }>;

/// Type for an ASCII string whose length is not predefined but bounded by a constant
///
/// The const generic `N` specifies the maxmimum number of ASCII characters a value of this
/// type is allowed to have.
#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LimitedString<const N: i32>(Vec<u8>);

impl<const N: i32> LimitedString<N> {
    /// Construct a new `LimitedString` from a byte vector
    ///
    /// The byte vector represents an ASCII string.
    /// The length of the byte vector must not exceed `N`. Otherwise this function returns
    /// an error
    pub fn new(vec: Vec<u8>) -> Result<Self, StellarSdkError> {
        match vec.len() > N as usize {
            true => Err(StellarSdkError::ExceedsMaximumLength {
                requested_length: vec.len(),
                allowed_length: N,
            }),
            false => Ok(LimitedString(vec)),
        }
    }

    /// Returns a reference to the raw byte vector
    pub fn get_vec(&self) -> &Vec<u8> {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<const N: i32> XdrCodec for LimitedString<N> {
    /// The XDR encoder implementation for `LimitedString`
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) {
        write_stream.write_next_u32(self.0.len() as u32);
        write_stream.write_next_binary_data(&self.0[..]);
    }

    /// The XDR decoder implementation for `LimitedString`
    fn from_xdr_buffered<R: AsRef<[u8]>>(
        read_stream: &mut ReadStream<R>,
    ) -> Result<Self, DecodeError> {
        let length = read_stream.read_next_u32()? as i32;
        match length > N {
            true => Err(DecodeError::StringExceedsMaxLength {
                at_position: read_stream.get_position(),
                max_length: N,
                actual_length: length,
            }),
            false => Ok(
                LimitedString::new(read_stream.read_next_binary_data(length as usize)?).unwrap(),
            ),
        }
    }
}

/// Type for an ASCII string whose length is not predefined and not bounded
///
/// Actually an `UnlimitedString` is limited: it must not have more than
/// `i32::MAX` characters.
#[allow(dead_code)]
pub type UnlimitedString = LimitedString<{ i32::MAX }>;

/// Type for an array whose length is not predefined but bounded by a constant
///
/// The generic variable `T` specifies the types of the elements of this array.
/// The const generic `N` specifies the maxmimum number of elements a value of this
/// type is allowed to have.
#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LimitedVarArray<T, const N: i32>(Vec<T>);

impl<T, const N: i32> LimitedVarArray<T, N> {
    /// Construct a new `LimitedVarArray` from a vector
    ///
    /// The length of the vector must not exceed `N`. Otherwise this function returns
    /// an error
    pub fn new(vec: Vec<T>) -> Result<Self, StellarSdkError> {
        match vec.len() > N as usize {
            true => Err(StellarSdkError::ExceedsMaximumLength {
                requested_length: vec.len(),
                allowed_length: N,
            }),
            false => Ok(LimitedVarArray(vec)),
        }
    }

    pub fn new_empty() -> Self {
        LimitedVarArray(vec![])
    }

    /// Returns a reference to the byte vector
    pub fn get_vec(&self) -> &Vec<T> {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Add an element to the byte vector
    ///
    /// Return an `Err` if the array already has the maximal number of elements.
    pub fn push(&mut self, item: T) -> Result<(), StellarSdkError> {
        if self.0.len() >= N as usize - 1 {
            return Err(StellarSdkError::ExceedsMaximumLength {
                requested_length: self.0.len() + 1,
                allowed_length: N,
            });
        }

        self.0.push(item);
        Ok(())
    }
}

impl<T: XdrCodec, const N: i32> XdrCodec for LimitedVarArray<T, N> {
    /// The XDR encoder implementation for `LimitedVarArray`
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) {
        write_stream.write_next_u32(self.0.len() as u32);
        for item in self.0.iter() {
            item.to_xdr_buffered(write_stream);
        }
    }

    /// The XDR decoder implementation for `LimitedVarArray`
    fn from_xdr_buffered<R: AsRef<[u8]>>(
        read_stream: &mut ReadStream<R>,
    ) -> Result<Self, DecodeError> {
        let length = read_stream.read_next_u32()? as i32;
        match length > N {
            true => Err(DecodeError::VarArrayExceedsMaxLength {
                at_position: read_stream.get_position(),
                max_length: N,
                actual_length: length,
            }),
            false => {
                let mut result = Vec::<T>::with_capacity(length as usize);
                for _ in 0..length {
                    result.push(T::from_xdr_buffered(read_stream)?)
                }
                Ok(LimitedVarArray::new(result).unwrap())
            }
        }
    }
}

/// Type for an XDR array whose length is not predefined and not bounded
///
/// Actually an `UnlimitedVarArray` is limited: it must not have more than
/// `i32::MAX` characters.
#[allow(dead_code)]
pub type UnlimitedVarArray<T> = LimitedVarArray<T, { i32::MAX }>;
