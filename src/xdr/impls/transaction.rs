//! Transaction envelopes and signatures

use crate::{
    types::{
        FeeBumpTransactionEnvelope, FeeBumpTransactionExt, FeeBumpTransactionInnerTx, Memo,
        MuxedAccount, Operation, Preconditions, TimeBounds, Transaction, TransactionEnvelope,
        TransactionExt, TransactionV0, TransactionV0Ext, TransactionV1Envelope,
    },
    xdr::compound_types::LimitedVarArray,
    FeeBumpTransaction, IntoAmount, IntoMuxedAccountId, StellarSdkError, BASE_FEE_STROOPS,
};

// fee is not fee per operation but total fee
// sequence_number must be 1 + current account sequence numbere
impl Transaction {
    pub fn new<T: IntoMuxedAccountId>(
        source_account: T,
        sequence_number: i64,
        fee_per_operation: Option<u32>,
        preconditions: Preconditions,
        memo: Option<Memo>,
    ) -> Result<Self, StellarSdkError> {
        let transaction = Self {
            source_account: source_account.into_muxed_account_id()?,
            fee: fee_per_operation.unwrap_or(BASE_FEE_STROOPS),
            seq_num: sequence_number,
            cond: preconditions,
            memo: memo.unwrap_or(Memo::MemoNone),
            operations: LimitedVarArray::new_empty(),
            ext: TransactionExt::V0,
        };

        Ok(transaction)
    }

    pub fn append_operation(&mut self, operation: Operation) -> Result<(), StellarSdkError> {
        self.operations.push(operation)
    }

    // careful: this operation also multiplies the fees with the number of operations
    pub fn into_transaction_envelope(mut self) -> TransactionEnvelope {
        self.fee = self
            .fee
            .checked_mul(self.operations.len() as u32)
            .unwrap_or(self.fee);

        TransactionEnvelope::EnvelopeTypeTx(TransactionV1Envelope {
            tx: self,
            signatures: LimitedVarArray::new_empty(),
        })
    }
}

impl From<TransactionV0> for Transaction {
    fn from(transaction: TransactionV0) -> Self {
        let time_bounds = transaction
            .time_bounds
            .unwrap_or(TimeBounds::from_time_points((), ()));

        Self {
            source_account: MuxedAccount::KeyTypeEd25519(transaction.source_account_ed25519),
            fee: transaction.fee,
            seq_num: transaction.seq_num,
            cond: Preconditions::PrecondTime(time_bounds),
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

#[cfg(test)]
mod test {
    use crate::{
        network::TEST_NETWORK, types::Preconditions, Asset, IntoSecretKey, Memo,
        MilliSecondEpochTime, Operation, Price, SecondEpochTime, StroopAmount, TimeBounds,
        Transaction, TransactionEnvelope, XdrCodec,
    };

    const ACCOUNT_ID1: &str = "GDGRDTRINPF66FNC47H22NY6BNWMCD5Q4XZTVA2KG7PFZ64WHRIU62TQ";
    const ACCOUNT_ID2: &str = "GBNKQVTFRP25TIQRODMU5GJGSXDKHCEUDN7LNMOS5PNM427LMR77NV4M";
    const ACCOUNT_ID3: &str = "GCACWDM2VEYTXGUI3CUYLBJ453IBEPQ3XEJKA772ARAP5XDQ4NMGFZGJ";

    const SIGNER1: &str = "SCVKZEONBSU3XD6OTHXGAP6BTEWHOU4RPZQZJJ5AVAGPXUZ5A4D7MU6S";
    const SIGNER3: &str = "SDOKV37I4TI655LMEMDQFOWESJ3LK6DDFKIVTYKN4YYTSAYFIBPP7MYI";

    #[test]
    fn build_transaction1() {
        let mut transaction = Transaction::new(
            ACCOUNT_ID1,
            1980190376853505,
            Some(100),
            Preconditions::PrecondTime(TimeBounds::from_time_points(
                SecondEpochTime(0),
                SecondEpochTime(1626258131),
            )),
            None,
        )
        .unwrap();

        transaction
            .append_operation(
                Operation::new_payment(ACCOUNT_ID2, Asset::native(), "123.456").unwrap(),
            )
            .unwrap();

        let expexted_base64 = b"AAAAAgAAAADNEc4oa8vvFaL\
            nz603HgtswQ+w5fM6g0o33lz7ljxRTwAAAGQABwj5AAAAAQAAAAEA\
            AAAAAAAAAAAAAABg7rrTAAAAAAAAAAEAAAAAAAAAAQAAAABaqFZli\
            /XZohFw2U6ZJpXGo4iUG362sdLr2s5r62R/9gAAAAAAAAAASZXkAA\
            AAAAAAAAAA";
        let envelope = transaction.into_transaction_envelope();

        assert_eq!(
            envelope,
            TransactionEnvelope::from_base64_xdr(expexted_base64).unwrap()
        );
        assert_eq!(envelope.to_base64_xdr(), expexted_base64);
    }

    #[test]
    fn build_transaction2() {
        let mut transaction = Transaction::new(
            ACCOUNT_ID1,
            1980190376853505,
            Some(321),
            Preconditions::PrecondTime(TimeBounds::from_time_points(
                SecondEpochTime(0),
                SecondEpochTime(0),
            )),
            Some(Memo::from_text_memo("Hello World!").unwrap()),
        )
        .unwrap();

        transaction
            .append_operation(
                Operation::new_payment(
                    ACCOUNT_ID2,
                    Asset::from_asset_code("USD", ACCOUNT_ID3).unwrap(),
                    StroopAmount(1234560000),
                )
                .unwrap(),
            )
            .unwrap();

        let expexted_base64 = b"AAAAAgAAAADNEc4oa8vvFaLnz603HgtswQ+w5fM6g0o33lz7ljxRTwAAAUEABwj5AAAAAQAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAxIZWxsbyBXb3JsZCEAAAABAAAAAAAAAAEAAAAAWqhWZYv12aIRcNlOmSaVxqOIlBt+trHS69rOa+tkf/YAAAABVVNEAAAAAACAKw2aqTE7mojYqYWFPO7QEj4buRKgf/oEQP7ccONYYgAAAABJleQAAAAAAAAAAAA=";
        let envelope = transaction.into_transaction_envelope();

        assert_eq!(
            envelope,
            TransactionEnvelope::from_base64_xdr(expexted_base64).unwrap()
        );
        assert_eq!(envelope.to_base64_xdr(), expexted_base64);
    }

    #[test]
    fn build_transaction3() {
        let mut transaction = Transaction::new(
            ACCOUNT_ID1,
            1980190376853505,
            Some(321),
            Preconditions::PrecondTime(TimeBounds::from_time_points(
                SecondEpochTime(162620000),
                MilliSecondEpochTime(1626263454_000),
            )),
            Some(Memo::from_text_memo("Hello World!").unwrap()),
        )
        .unwrap();

        transaction
            .append_operation(
                Operation::new_payment(
                    ACCOUNT_ID2,
                    Asset::from_asset_code("USD", ACCOUNT_ID3).unwrap(),
                    StroopAmount(1234560000),
                )
                .unwrap()
                .set_source_account(ACCOUNT_ID3)
                .unwrap(),
            )
            .unwrap();

        transaction
            .append_operation(
                Operation::new_manage_sell_offer(
                    Asset::from_asset_code("DOMINATION", ACCOUNT_ID2).unwrap(),
                    Asset::native(),
                    "152.103",
                    Price::from_float(4.58).unwrap(),
                    Some(123456789),
                )
                .unwrap(),
            )
            .unwrap();

        let expexted_base64 = b"AAAAAgAAAADNEc4oa8vvFaLnz603HgtswQ+w5fM6g0o33lz7ljxRTwAAAoIABwj5AAAAAQAAAAEAAAAACbFiYAAAAABg7s+eAAAAAQAAAAxIZWxsbyBXb3JsZCEAAAACAAAAAQAAAACAKw2aqTE7mojYqYWFPO7QEj4buRKgf/oEQP7ccONYYgAAAAEAAAAAWqhWZYv12aIRcNlOmSaVxqOIlBt+trHS69rOa+tkf/YAAAABVVNEAAAAAACAKw2aqTE7mojYqYWFPO7QEj4buRKgf/oEQP7ccONYYgAAAABJleQAAAAAAAAAAAMAAAACRE9NSU5BVElPTgAAAAAAAFqoVmWL9dmiEXDZTpkmlcajiJQbfrax0uvazmvrZH/2AAAAAAAAAABaqRNwAAAA5QAAADIAAAAAB1vNFQAAAAAAAAAA";
        let mut envelope = transaction.into_transaction_envelope();

        assert_eq!(
            envelope,
            TransactionEnvelope::from_base64_xdr(expexted_base64).unwrap()
        );
        assert_eq!(envelope.to_base64_xdr(), expexted_base64);

        envelope
            .sign(
                &TEST_NETWORK,
                vec![
                    &SIGNER1.into_secret_key().unwrap(),
                    &SIGNER3.into_secret_key().unwrap(),
                ],
            )
            .unwrap();

        let expexted_singed_base64 = b"AAAAAgAAAADNEc4oa8vvFaLnz603HgtswQ+w5fM6g0o33lz7ljxRTwAAAoIABwj5AAAAAQAAAAEAAAAACbFiYAAAAABg7s+eAAAAAQAAAAxIZWxsbyBXb3JsZCEAAAACAAAAAQAAAACAKw2aqTE7mojYqYWFPO7QEj4buRKgf/oEQP7ccONYYgAAAAEAAAAAWqhWZYv12aIRcNlOmSaVxqOIlBt+trHS69rOa+tkf/YAAAABVVNEAAAAAACAKw2aqTE7mojYqYWFPO7QEj4buRKgf/oEQP7ccONYYgAAAABJleQAAAAAAAAAAAMAAAACRE9NSU5BVElPTgAAAAAAAFqoVmWL9dmiEXDZTpkmlcajiJQbfrax0uvazmvrZH/2AAAAAAAAAABaqRNwAAAA5QAAADIAAAAAB1vNFQAAAAAAAAACljxRTwAAAEB0B8vODxIESpa9H9f4QkPtFHVg4Xjx2A9aTncJOkW6BW0i1AxZFgMvrzEb7nO5UnXRvCKnBmuhpvA76YivAXYGcONYYgAAAECesooI2hhhuoOcLcXB76L58vMOrFvPFqpIeG+/zzLZXz0XYU6aELdtDxLAhK8GZCIZwlXdJ/RyZF9/2YzqbPMC";
        assert_eq!(
            envelope,
            TransactionEnvelope::from_base64_xdr(expexted_singed_base64).unwrap()
        );
        assert_eq!(envelope.to_base64_xdr(), expexted_singed_base64);
    }
}
