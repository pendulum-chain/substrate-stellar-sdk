use serde_json::Value;
use sp_runtime::offchain::http::Method;
use sp_std::{prelude::*, vec::Vec};

use super::{FetchError, Horizon};
use crate::{
    types::{OperationBody, Uint256},
    utils::percent_encode::percent_encode,
    AccountId, Memo, MuxedAccount, TransactionEnvelope, XdrCodec,
};

// ACCOUNT_REQUIRES_MEMO is the base64 encoding of "1".
// SEP 29 uses this value to define transaction memo requirements for incoming payments.
const ACCOUNT_REQUIRES_MEMO: &str = "MQ==";

impl Horizon {
    // [SEP0029](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0029.md)
    fn check_if_memo_required(
        &self,
        transaction_envelope: &TransactionEnvelope,
        timeout_milliseconds: u64,
    ) -> Result<(), FetchError> {
        let (memo, operations) = match transaction_envelope {
            TransactionEnvelope::EnvelopeTypeTxV0(envelope) => {
                (&envelope.tx.memo, &envelope.tx.operations)
            }
            TransactionEnvelope::EnvelopeTypeTx(envelope) => {
                (&envelope.tx.memo, &envelope.tx.operations)
            }
            TransactionEnvelope::EnvelopeTypeTxFeeBump(envelope) => match &envelope.tx.inner_tx {
                crate::types::FeeBumpTransactionInnerTx::EnvelopeTypeTx(envelope) => {
                    (&envelope.tx.memo, &envelope.tx.operations)
                }
                crate::types::FeeBumpTransactionInnerTx::Default(_) => unreachable!(),
            },
            TransactionEnvelope::Default(_) => unreachable!(),
        };

        if *memo != Memo::MemoNone {
            return Ok(());
        }

        let mut destinations: Vec<&Uint256> = Vec::with_capacity(operations.len());

        for operation in operations.get_vec() {
            let destination = match &operation.body {
                OperationBody::Payment(body) => &body.destination,
                OperationBody::PathPaymentStrictReceive(body) => &body.destination,
                OperationBody::PathPaymentStrictSend(body) => &body.destination,
                OperationBody::AccountMerge(body) => body,
                _ => continue,
            };

            let destination = match destination {
                MuxedAccount::KeyTypeEd25519(destination) => destination,
                MuxedAccount::KeyTypeMuxedEd25519(_) => continue,
                MuxedAccount::Default(_) => unreachable!(),
            };

            if destinations.contains(&destination) {
                continue;
            }

            destinations.push(destination);

            let account_response = self.fetch_account(
                AccountId::from_binary(destination.clone()),
                timeout_milliseconds,
            );

            let account_response = match account_response {
                Ok(account_response) => account_response,
                Err(FetchError::UnexpectedResponseStatus { status: 404 }) => continue,
                Err(error) => return Err(error),
            };

            let map = match account_response.data {
                Value::Object(map) => map,
                _ => continue,
            };

            let data = match map.get("config.memo_required") {
                Some(Value::String(data)) => data,
                _ => continue,
            };

            if *data == ACCOUNT_REQUIRES_MEMO {
                return Err(FetchError::AccountRequiredMemo(AccountId::from_binary(
                    destination.clone(),
                )));
            }
        }

        Ok(())
    }

    // recommended timeout: 60_000;
    pub fn submit_transaction(
        &self,
        transaction_envelope: &TransactionEnvelope,
        timeout_milliseconds: u64,
        skip_memo_required_check: bool,
    ) -> Result<Value, FetchError> {
        if !skip_memo_required_check {
            self.check_if_memo_required(transaction_envelope, timeout_milliseconds)?;
        }

        let envelope_base64 = transaction_envelope.to_base64_xdr();
        let json = self.request(
            vec![b"transactions/tx=", &percent_encode(envelope_base64)[..]],
            Method::Post,
            timeout_milliseconds,
        )?;

        Ok(serde_json::from_slice(&json)?)
    }
}
