//! Generic types for encoding XDR variable length arrays and strings

use core::convert::AsRef;
use sp_std::{vec, vec::Vec};

use super::{
    streams::{DecodeError, ReadStream, WriteStream},
    xdr_codec::XdrCodec,
};
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
            true => Err(StellarSdkError::ExceedsMaximumLength { requested_length: vec.len(), allowed_length: N }),
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
    fn from_xdr_buffered<R: AsRef<[u8]>>(read_stream: &mut ReadStream<R>) -> Result<Self, DecodeError> {
        let length = read_stream.read_next_u32()? as i32;
        match length > N {
            true => Err(DecodeError::VarOpaqueExceedsMaxLength {
                at_position: read_stream.get_position(),
                max_length: N,
                actual_length: length,
            }),
            false => Ok(LimitedVarOpaque::new(read_stream.read_next_binary_data(length as usize)?).unwrap()),
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
            true => Err(StellarSdkError::ExceedsMaximumLength { requested_length: vec.len(), allowed_length: N }),
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
    fn from_xdr_buffered<R: AsRef<[u8]>>(read_stream: &mut ReadStream<R>) -> Result<Self, DecodeError> {
        let length = read_stream.read_next_u32()? as i32;
        match length > N {
            true => Err(DecodeError::StringExceedsMaxLength {
                at_position: read_stream.get_position(),
                max_length: N,
                actual_length: length,
            }),
            false => Ok(LimitedString::new(read_stream.read_next_binary_data(length as usize)?).unwrap()),
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
            true => Err(StellarSdkError::ExceedsMaximumLength { requested_length: vec.len(), allowed_length: N }),
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
            return Err(StellarSdkError::ExceedsMaximumLength { requested_length: self.0.len() + 1, allowed_length: N })
        }

        self.0.push(item);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
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
    fn from_xdr_buffered<R: AsRef<[u8]>>(read_stream: &mut ReadStream<R>) -> Result<Self, DecodeError> {
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
            },
        }
    }
}

/// Type for an XDR array whose length is not predefined and not bounded
///
/// Actually an `UnlimitedVarArray` is limited: it must not have more than
/// `i32::MAX` characters.
#[allow(dead_code)]
pub type UnlimitedVarArray<T> = LimitedVarArray<T, { i32::MAX }>;

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct XdrArchive<T>(Vec<T>);

impl<T> XdrArchive<T> {
    /// Construct a new `XdrArchive` from a vector
    pub fn new(vec: Vec<T>) -> Self {
        XdrArchive(vec)
    }

    pub fn new_empty() -> Self {
        XdrArchive(vec![])
    }

    /// Returns a reference to the byte vector
    pub fn get_vec(&self) -> &Vec<T> {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Add an element to the byte vector
    pub fn push(&mut self, item: T) -> () {
        self.0.push(item);
    }
}

impl<T: XdrCodec> XdrCodec for XdrArchive<T> {
    /// The XDR encoder implementation for `XdrArchive`
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) {
        for item in self.0.iter() {
            let item_xdr = item.to_xdr();
            let length = item_xdr.len();
            if length < 0x80_00_00_00 {
                write_stream.write_next_u32((length as u32) | 0x80_00_00_00);
                write_stream.write_next_binary_data(&item_xdr);
            }
        }
    }

    /// The XDR decoder implementation for `XdrArchive`
    fn from_xdr_buffered<R: AsRef<[u8]>>(read_stream: &mut ReadStream<R>) -> Result<Self, DecodeError> {
        let mut result = Vec::<T>::new();
        while read_stream.no_of_bytes_left_to_read() > 0 {
            let length = read_stream.read_next_u32()? & 0x7f_ff_ff_ff;
            let old_position = read_stream.get_position();

            result.push(T::from_xdr_buffered(read_stream)?);

            if read_stream.get_position() - old_position != length as usize {
                return Err(DecodeError::InvalidXdrArchiveLength { at_position: old_position })
            }
        }

        Ok(XdrArchive::new(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Price;

    #[test]
    fn keypair() {
        let xdr_archive = XdrArchive::<LimitedVarArray<Price, 10>>::new(vec![
            LimitedVarArray::new(vec![Price { n: 10, d: 3 }, Price { n: 1, d: 4 }]).unwrap(),
            LimitedVarArray::new(vec![Price { n: 5, d: 2 }]).unwrap(),
        ]);

        let encoded = xdr_archive.to_xdr();
        assert_eq!(
            encoded,
            vec![
                128, 0, 0, 20, 0, 0, 0, 2, 0, 0, 0, 10, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 4, 128, 0, 0, 12, 0, 0, 0, 1,
                0, 0, 0, 5, 0, 0, 0, 2
            ]
        );
        assert_eq!(XdrArchive::<LimitedVarArray<Price, 10>>::from_xdr(encoded).unwrap(), xdr_archive)
    }

    #[test]
    fn pop_limitedarray() {
        let sample_vec = vec![0,1,2,3,4];
        let mut sample_limited_array = LimitedVarArray::<u8,5>::new(sample_vec).expect("should return just fine");
        let len = sample_limited_array.len();
        let popped = sample_limited_array.pop();
        assert_eq!(popped, Some(4));
        assert_ne!(sample_limited_array.len(), len);
    }
}
