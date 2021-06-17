use core::convert::TryInto;
use sodalite::SIGN_LEN;
use sp_std::vec::Vec;
use substrate_stellar_xdr::{
    compound_types::{LimitedVarArray, LimitedVarOpaque},
    xdr::{self, DecoratedSignature},
    xdr_codec::XdrCodec,
};

use super::keypair::{Keypair, PublicKey};
use super::utils::sha256::sha256;
use crate::utils::{base64, network::Network, sha256::BinarySha256Hash};

pub enum SignatureError {
    InvalidLength {
        found_length: usize,
        expected_length: usize,
    },
    PublicKeyCantVerify,
    InvalidBase64Encoding,
    TooManySignatures,
}

fn get_signatures(
    transaction_envelope: &mut xdr::TransactionEnvelope,
) -> &mut LimitedVarArray<DecoratedSignature, 20> {
    match transaction_envelope {
        xdr::TransactionEnvelope::EnvelopeTypeTxV0(envelope) => &mut envelope.signatures,
        xdr::TransactionEnvelope::EnvelopeTypeTx(envelope) => &mut envelope.signatures,
        xdr::TransactionEnvelope::EnvelopeTypeTxFeeBump(envelope) => &mut envelope.signatures,
        _ => unreachable!("Invalid transaction envelope type"),
    }
}

pub fn create_base64_signature(
    transaction_envelope: &mut xdr::TransactionEnvelope,
    network: &Network,
    keypair: &Keypair,
) -> Vec<u8> {
    let transaction_hash = get_hash(transaction_envelope, network);
    let signature = keypair.create_signature(transaction_hash);
    base64::encode(signature)
}

pub fn sign(
    transaction_envelope: &mut xdr::TransactionEnvelope,
    network: &Network,
    keypairs: Vec<&Keypair>,
) -> Result<(), SignatureError> {
    let transaction_hash = get_hash(transaction_envelope, network);

    let signatures = get_signatures(transaction_envelope);

    for keypair in keypairs.iter() {
        let signature = keypair.create_signature(transaction_hash);
        let hint = keypair.get_public().get_signature_hint();

        signatures
            .push(xdr::DecoratedSignature {
                hint,
                signature: LimitedVarOpaque::new(Vec::from(signature)).unwrap(),
            })
            .map_err(|_| SignatureError::TooManySignatures)?;
    }

    Ok(())
}

pub fn add_signature<T: AsRef<[u8]>>(
    transaction_envelope: &mut xdr::TransactionEnvelope,
    network: &Network,
    base64_signature: T,
    public_key: &PublicKey,
) -> Result<(), SignatureError> {
    let signature = match base64::decode(base64_signature) {
        Err(_) => Err(SignatureError::InvalidBase64Encoding)?,
        Ok(signature) => {
            if signature.len() != SIGN_LEN {
                return Err(SignatureError::InvalidLength {
                    found_length: signature.len(),
                    expected_length: SIGN_LEN,
                });
            };
            signature
        }
    };

    let transaction_hash = get_hash(transaction_envelope, network);
    if !public_key.verify_signature(transaction_hash, signature[..].try_into().unwrap()) {
        return Err(SignatureError::PublicKeyCantVerify);
    }

    let signatures = get_signatures(transaction_envelope);

    signatures
        .push(xdr::DecoratedSignature {
            hint: public_key.get_signature_hint(),
            signature: LimitedVarOpaque::new(signature).unwrap(),
        })
        .map_err(|_| SignatureError::TooManySignatures)?;

    Ok(())
}

pub fn get_hash(
    transaction_envelope: &xdr::TransactionEnvelope,
    network: &Network,
) -> BinarySha256Hash {
    let network_id = network.get_id().clone();

    let tagged_transaction = match transaction_envelope {
        xdr::TransactionEnvelope::EnvelopeTypeTxV0(transaction_envelope) => {
            let transaction = xdr::Transaction {
                source_account: xdr::MuxedAccount::KeyTypeEd25519(
                    transaction_envelope.tx.source_account_ed25519,
                ),
                fee: transaction_envelope.tx.fee,
                seq_num: transaction_envelope.tx.seq_num,
                time_bounds: transaction_envelope.tx.time_bounds.clone(),
                memo: transaction_envelope.tx.memo.clone(),
                operations: transaction_envelope.tx.operations.clone(),
                ext: match transaction_envelope.tx.ext {
                    xdr::TransactionV0Ext::V0 => xdr::TransactionExt::V0,
                    xdr::TransactionV0Ext::Default(default) => {
                        xdr::TransactionExt::Default(default)
                    }
                },
            };
            xdr::TransactionSignaturePayloadTaggedTransaction::EnvelopeTypeTx(transaction)
        }

        xdr::TransactionEnvelope::EnvelopeTypeTx(transaction_envelope) => {
            xdr::TransactionSignaturePayloadTaggedTransaction::EnvelopeTypeTx(
                transaction_envelope.tx.clone(),
            )
        }

        xdr::TransactionEnvelope::EnvelopeTypeTxFeeBump(transaction_envelope) => {
            xdr::TransactionSignaturePayloadTaggedTransaction::EnvelopeTypeTxFeeBump(
                transaction_envelope.tx.clone(),
            )
        }

        _ => unimplemented!("This type of transaction envelope is not supported"),
    };

    let signature_payload = xdr::TransactionSignaturePayload {
        network_id,
        tagged_transaction,
    };

    sha256(signature_payload.to_xdr())
}

#[cfg(test)]
mod tests {
    use sp_std::{prelude::*, vec::Vec};

    use substrate_stellar_xdr::{
        compound_types::LimitedVarArray,
        xdr::{
            Asset, Memo, MuxedAccount, Operation, OperationBody, PaymentOp, TimeBounds,
            Transaction, TransactionEnvelope, TransactionExt, TransactionV1Envelope,
        },
        xdr_codec::XdrCodec,
    };

    use crate::{
        keypair::{Keypair, PublicKey},
        transaction::sign,
        utils::network::TEST_NETWORK,
    };

    #[test]
    fn keypair() {
        let secret = "SCDSVACTNFNSD5LQZ5LWUWEY3UIAML2J7ALPFCD6ZX4D3TVJV7X243N3";
        let keypair = Keypair::from_encoded_secret(secret);
        assert!(keypair.is_ok());
        let keypair = keypair.unwrap();

        let dest_public =
            PublicKey::from_encoding("GDMTKCJQ322RDTGOBLIPVEUCO3EIEJLXDV4JTWLXU6AFOYTMSJ45WZY5")
                .unwrap();

        let mut transaction_envelope = TransactionEnvelope::EnvelopeTypeTx(TransactionV1Envelope {
            tx: Transaction {
                source_account: MuxedAccount::KeyTypeEd25519(
                    keypair.get_public().get_binary().clone(),
                ),
                fee: 10000,
                seq_num: 59481002082305,
                time_bounds: Some(TimeBounds {
                    min_time: 0,
                    max_time: 0,
                }),
                memo: Memo::MemoNone,
                operations: LimitedVarArray::new(vec![Operation {
                    source_account: None,
                    body: OperationBody::Payment(PaymentOp {
                        destination: MuxedAccount::KeyTypeEd25519(dest_public.get_binary().clone()),
                        asset: Asset::AssetTypeNative,
                        amount: 10_000_000,
                    }),
                }])
                .unwrap(),
                ext: TransactionExt::V0,
            },
            signatures: LimitedVarArray::new(Vec::new()).unwrap(),
        });

        let expected_xdr = "AAAAAgAAAABRVWJF9F/Kd+p+e65fn2mnDGH5BnlL9yXAMBTaJbnUcQAAJxAAADYZAAAAAQAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAAAQAAAADZNQkw3rURzM4K0PqSgnbIgiV3HXiZ2XengFdibJJ52wAAAAAAAAAAAJiWgAAAAAAAAAAA";
        assert_eq!(
            transaction_envelope.to_xdr().as_slice(),
            crate::utils::base64::decode(expected_xdr)
                .unwrap()
                .as_slice()
        );

        let signing_result = sign(&mut transaction_envelope, &TEST_NETWORK, vec![&keypair]);
        assert!(signing_result.is_ok());

        let expected_signed_xdr = "AAAAAgAAAABRVWJF9F/Kd+p+e65fn2mnDGH5BnlL9yXAMBTaJbnUcQAAAGQAADYZAAAAAQAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAAAQAAAADZNQkw3rURzM4K0PqSgnbIgiV3HXiZ2XengFdibJJ52wAAAAAAAAAAAJiWgAAAAAAAAAABJbnUcQAAAEAMYswJPUbulgvFqFdfJcDMKX4iP7MSetBLrqYO6QRpl91zoHQ0IzQhhgeR4e6N3kLLy/vBFLVEb4ZsY+sfJlYO";

        assert_eq!(
            transaction_envelope.to_xdr().as_slice(),
            crate::utils::base64::decode(expected_signed_xdr)
                .unwrap()
                .as_slice()
        );
    }
}
