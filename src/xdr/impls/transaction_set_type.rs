#![allow(dead_code)]
use crate::{Hash, IntoHash, ReadStream, TransactionEnvelope, WriteStream, XdrCodec};
use crate::compound_types::UnlimitedVarArray;
use crate::types::{GeneralizedTransactionSet, TransactionSet};
use crate::xdr::streams::DecodeError;

pub enum TransactionSetType {
    TransactionSet(TransactionSet),
    GeneralizedTransactionSet(GeneralizedTransactionSet),
}

impl TransactionSetType {
    pub fn get_tx_set_hash(&self) -> Result<Hash, ()> {
        match self {
            TransactionSetType::TransactionSet(tx_set) =>
                tx_set.clone().into_hash().map_err(|_| ()),
            TransactionSetType::GeneralizedTransactionSet(tx_set) =>
                tx_set.clone().into_hash().map_err(|_| ()),
        }
    }

    pub fn txes(&self) -> UnlimitedVarArray<TransactionEnvelope> {
        let txes_option = match self {
            TransactionSetType::TransactionSet(tx_set) => Some(tx_set.txes.clone()),
            TransactionSetType::GeneralizedTransactionSet(tx_set) => tx_set.txes(),
        };

        txes_option.unwrap_or_else(UnlimitedVarArray::new_empty)
    }
}

impl XdrCodec for TransactionSetType {
    fn to_xdr_buffered(&self, write_stream: &mut WriteStream) {
        match self {
            TransactionSetType::TransactionSet(set) => {
                (0 as i32).to_xdr_buffered(write_stream);
                set.to_xdr_buffered(write_stream)
            }
            TransactionSetType::GeneralizedTransactionSet(set) => {
                (1 as i32).to_xdr_buffered(write_stream);
                set.to_xdr_buffered(write_stream)
            }
        }
    }

    fn from_xdr_buffered<T: AsRef<[u8]>>(read_stream: &mut ReadStream<T>) -> Result<Self, DecodeError> {
        match i32::from_xdr_buffered(read_stream)? {
            0 => Ok(TransactionSetType::TransactionSet(TransactionSet::from_xdr_buffered(read_stream)?)),
            1 => Ok(TransactionSetType::GeneralizedTransactionSet(GeneralizedTransactionSet::from_xdr_buffered(read_stream)?)),
            _ => Err(DecodeError::InvalidEnumDiscriminator { at_position: 0 })
        }
    }
}

