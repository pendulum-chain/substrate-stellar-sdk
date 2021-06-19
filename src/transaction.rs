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
            self, Asset, AssetAlphaNum4, ManageSellOfferOp, Memo, MuxedAccount, Operation,
            OperationBody, PaymentOp, Price, TimeBounds, Transaction, TransactionEnvelope,
            TransactionExt, TransactionV1Envelope, Uint256,
        },
        xdr_codec::XdrCodec,
    };

    use crate::{
        keypair::{Keypair, PublicKey},
        transaction::sign,
        utils::network::TEST_NETWORK,
    };

    fn binary_public(public: &str) -> Uint256 {
        PublicKey::from_encoding(public)
            .unwrap()
            .get_binary()
            .clone()
    }

    #[test]
    fn decode_complex_transaction() {
        let envelope = "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";

        let envelope =
            TransactionEnvelope::from_xdr(crate::utils::base64::decode(envelope).unwrap());
        assert!(envelope.is_ok());
        let envelope = match envelope.unwrap() {
            TransactionEnvelope::EnvelopeTypeTx(envelope) => envelope,
            _ => unreachable!(),
        };

        let transaction = envelope.tx;

        let expected_transaction = Transaction {
            source_account: MuxedAccount::KeyTypeEd25519(binary_public(
                "GAD5EVNWATBLSGQY4MT2V2Y3RRN47N742LYSGTSDM4QN6PNMXH2LF7WV",
            )),
            fee: 1500,
            seq_num: 153882209995006140,
            time_bounds: Some(TimeBounds {
                min_time: 0,
                max_time: 0,
            }),
            memo: Memo::MemoNone,
            operations: LimitedVarArray::new(vec![
                Operation {
                    source_account: None,
                    body: OperationBody::ManageSellOffer(ManageSellOfferOp {
                        selling: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GCQTGZQQ5G4PTM2GL7CDIFKUBIPEC52BROAQIAPW53XBRJVN6ZJVTG6V",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"LTC\0".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        amount: 1964075700,
                        price: Price {
                            n: 12751,
                            d: 2000000,
                        },
                        offer_id: 643129013,
                    }),
                },
                Operation {
                    source_account: None,
                    body: OperationBody::ManageSellOffer(ManageSellOfferOp {
                        selling: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GCQTGZQQ5G4PTM2GL7CDIFKUBIPEC52BROAQIAPW53XBRJVN6ZJVTG6V",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"LTC\0".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        amount: 2352600000,
                        price: Price {
                            n: 63759,
                            d: 10000000,
                        },
                        offer_id: 643590537,
                    }),
                },
                Operation {
                    source_account: None,
                    body: OperationBody::ManageSellOffer(ManageSellOfferOp {
                        selling: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GCQTGZQQ5G4PTM2GL7CDIFKUBIPEC52BROAQIAPW53XBRJVN6ZJVTG6V",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"LTC\0".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        amount: 2352450000,
                        price: Price {
                            n: 63763,
                            d: 10000000,
                        },
                        offer_id: 643718766,
                    }),
                },
                Operation {
                    source_account: None,
                    body: OperationBody::ManageSellOffer(ManageSellOfferOp {
                        selling: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GCQTGZQQ5G4PTM2GL7CDIFKUBIPEC52BROAQIAPW53XBRJVN6ZJVTG6V",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"LTC\0".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        amount: 2352300000,
                        price: Price {
                            n: 63767,
                            d: 10000000,
                        },
                        offer_id: 644253601,
                    }),
                },
                Operation {
                    source_account: None,
                    body: OperationBody::ManageSellOffer(ManageSellOfferOp {
                        selling: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GCQTGZQQ5G4PTM2GL7CDIFKUBIPEC52BROAQIAPW53XBRJVN6ZJVTG6V",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"LTC\0".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        amount: 2352150000,
                        price: Price {
                            n: 63771,
                            d: 10000000,
                        },
                        offer_id: 644960245,
                    }),
                },
                Operation {
                    source_account: None,
                    body: OperationBody::ManageSellOffer(ManageSellOfferOp {
                        selling: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"LTC\0".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GCQTGZQQ5G4PTM2GL7CDIFKUBIPEC52BROAQIAPW53XBRJVN6ZJVTG6V",
                            )),
                        }),
                        amount: 1537200,
                        price: Price { n: 15929, d: 100 },
                        offer_id: 643510837,
                    }),
                },
                Operation {
                    source_account: None,
                    body: OperationBody::ManageSellOffer(ManageSellOfferOp {
                        selling: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"LTC\0".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GCQTGZQQ5G4PTM2GL7CDIFKUBIPEC52BROAQIAPW53XBRJVN6ZJVTG6V",
                            )),
                        }),
                        amount: 7352800,
                        price: Price { n: 15931, d: 100 },
                        offer_id: 643542126,
                    }),
                },
                Operation {
                    source_account: None,
                    body: OperationBody::ManageSellOffer(ManageSellOfferOp {
                        selling: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"LTC\0".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GCQTGZQQ5G4PTM2GL7CDIFKUBIPEC52BROAQIAPW53XBRJVN6ZJVTG6V",
                            )),
                        }),
                        amount: 15000000,
                        price: Price { n: 3983, d: 25 },
                        offer_id: 643542127,
                    }),
                },
                Operation {
                    source_account: None,
                    body: OperationBody::ManageSellOffer(ManageSellOfferOp {
                        selling: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"LTC\0".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GCQTGZQQ5G4PTM2GL7CDIFKUBIPEC52BROAQIAPW53XBRJVN6ZJVTG6V",
                            )),
                        }),
                        amount: 15000000,
                        price: Price { n: 15933, d: 100 },
                        offer_id: 644085620,
                    }),
                },
                Operation {
                    source_account: None,
                    body: OperationBody::ManageSellOffer(ManageSellOfferOp {
                        selling: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"LTC\0".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GCQTGZQQ5G4PTM2GL7CDIFKUBIPEC52BROAQIAPW53XBRJVN6ZJVTG6V",
                            )),
                        }),
                        amount: 15000000,
                        price: Price { n: 7967, d: 50 },
                        offer_id: 644085621,
                    }),
                },
                Operation {
                    source_account: None,
                    body: OperationBody::ManageSellOffer(ManageSellOfferOp {
                        selling: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"LTC\0".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GCQTGZQQ5G4PTM2GL7CDIFKUBIPEC52BROAQIAPW53XBRJVN6ZJVTG6V",
                            )),
                        }),
                        amount: 14999900,
                        price: Price { n: 3187, d: 20 },
                        offer_id: 644085622,
                    }),
                },
                Operation {
                    source_account: None,
                    body: OperationBody::ManageSellOffer(ManageSellOfferOp {
                        selling: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"LTC\0".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GCQTGZQQ5G4PTM2GL7CDIFKUBIPEC52BROAQIAPW53XBRJVN6ZJVTG6V",
                            )),
                        }),
                        amount: 14999900,
                        price: Price { n: 3984, d: 25 },
                        offer_id: 644485720,
                    }),
                },
                Operation {
                    source_account: None,
                    body: OperationBody::ManageSellOffer(ManageSellOfferOp {
                        selling: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"LTC\0".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GCQTGZQQ5G4PTM2GL7CDIFKUBIPEC52BROAQIAPW53XBRJVN6ZJVTG6V",
                            )),
                        }),
                        amount: 15000000,
                        price: Price { n: 15937, d: 100 },
                        offer_id: 644508302,
                    }),
                },
                Operation {
                    source_account: None,
                    body: OperationBody::ManageSellOffer(ManageSellOfferOp {
                        selling: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"LTC\0".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GCQTGZQQ5G4PTM2GL7CDIFKUBIPEC52BROAQIAPW53XBRJVN6ZJVTG6V",
                            )),
                        }),
                        amount: 15000000,
                        price: Price { n: 7969, d: 50 },
                        offer_id: 644508754,
                    }),
                },
                Operation {
                    source_account: None,
                    body: OperationBody::ManageSellOffer(ManageSellOfferOp {
                        selling: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"LTC\0".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: xdr::PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GCQTGZQQ5G4PTM2GL7CDIFKUBIPEC52BROAQIAPW53XBRJVN6ZJVTG6V",
                            )),
                        }),
                        amount: 14999900,
                        price: Price { n: 15939, d: 100 },
                        offer_id: 644933867,
                    }),
                },
            ])
            .unwrap(),
            ext: TransactionExt::V0,
        };

        assert_eq!(transaction, expected_transaction);
    }

    #[test]
    fn sign_simple_transaction() {
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

        let expected_signed_xdr = "AAAAAgAAAABRVWJF9F/Kd+p+e65fn2mnDGH5BnlL9yXAMBTaJbnUcQAAJxAAADYZAAAAAQAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAAAQAAAADZNQkw3rURzM4K0PqSgnbIgiV3HXiZ2XengFdibJJ52wAAAAAAAAAAAJiWgAAAAAAAAAABJbnUcQAAAEAvCLQxbuE/zeBYq5Q/17d1hvcQME5uHUJ9SE8L8E/PQHa00jfGpFrtsG+XQV0DI0AnnqQhBhHKl1l5LNpIoxIA";

        assert_eq!(
            transaction_envelope.to_xdr().as_slice(),
            crate::utils::base64::decode(expected_signed_xdr)
                .unwrap()
                .as_slice()
        );
    }
}
