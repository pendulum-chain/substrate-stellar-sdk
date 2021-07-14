//! Transaction envelopes and signatures

use crate::{
    types::{
        FeeBumpTransactionEnvelope, FeeBumpTransactionExt, FeeBumpTransactionInnerTx, Memo,
        MuxedAccount, Operation, TimeBounds, Transaction, TransactionEnvelope, TransactionExt,
        TransactionV0, TransactionV0Ext, TransactionV1Envelope,
    },
    xdr::compound_types::LimitedVarArray,
    FeeBumpTransaction, IntoAmount, IntoMuxedAccountId, StellarSdkError, BASE_FEE_STROOPS,
};

// fee is not fee per operation but total fee
// sequence_number must be 1 + current account sequence numbere
impl Transaction {
    pub fn new<T: IntoMuxedAccountId>(
        source_account: T,
        current_sequence_number: i64,
        total_fee: Option<u32>,
        time_bounds: Option<TimeBounds>,
        memo: Option<Memo>,
    ) -> Result<Self, StellarSdkError> {
        let transaction = Self {
            source_account: source_account.into_muxed_account_id()?,
            fee: total_fee.unwrap_or(BASE_FEE_STROOPS),
            seq_num: current_sequence_number + 1,
            time_bounds,
            memo: memo.unwrap_or(Memo::MemoNone),
            operations: LimitedVarArray::new_empty(),
            ext: TransactionExt::V0,
        };

        Ok(transaction)
    }

    pub fn append_operation(&mut self, operation: Operation) -> Result<(), StellarSdkError> {
        self.operations.push(operation)
    }

    pub fn into_transaction_envelope(self) -> TransactionEnvelope {
        TransactionEnvelope::EnvelopeTypeTx(TransactionV1Envelope {
            tx: self,
            signatures: LimitedVarArray::new_empty(),
        })
    }
}

impl From<TransactionV0> for Transaction {
    fn from(transaction: TransactionV0) -> Self {
        Self {
            source_account: MuxedAccount::KeyTypeEd25519(transaction.source_account_ed25519),
            fee: transaction.fee,
            seq_num: transaction.seq_num,
            time_bounds: transaction.time_bounds,
            memo: transaction.memo,
            operations: transaction.operations,
            ext: match transaction.ext {
                TransactionV0Ext::V0 => TransactionExt::V0,
                TransactionV0Ext::Default(default) => TransactionExt::Default(default),
            },
        }
    }
}

impl FeeBumpTransaction {
    pub fn new<T: IntoMuxedAccountId, S: IntoAmount>(
        fee_source: T,
        new_total_fee: S,
        inner_transaction_envelope: TransactionEnvelope,
    ) -> Result<Self, StellarSdkError> {
        let v1_envelope = match inner_transaction_envelope {
            TransactionEnvelope::EnvelopeTypeTxV0(envelope) => TransactionV1Envelope {
                tx: envelope.tx.into(),
                signatures: envelope.signatures,
            },
            TransactionEnvelope::EnvelopeTypeTx(envelope) => envelope,
            TransactionEnvelope::EnvelopeTypeTxFeeBump(_) => {
                return Err(StellarSdkError::CantWrapFeeBumpTransaction)
            }
            TransactionEnvelope::Default(_) => unreachable!(),
        };

        let transaction = Self {
            fee_source: fee_source.into_muxed_account_id()?,
            fee: new_total_fee.into_stroop_amount(false)?,
            inner_tx: FeeBumpTransactionInnerTx::EnvelopeTypeTx(v1_envelope),
            ext: FeeBumpTransactionExt::V0,
        };

        Ok(transaction)
    }

    pub fn into_transaction_envelope(self) -> TransactionEnvelope {
        TransactionEnvelope::EnvelopeTypeTxFeeBump(FeeBumpTransactionEnvelope {
            tx: self,
            signatures: LimitedVarArray::new_empty(),
        })
    }
}
