//! Transaction envelopes and signatures

use core::convert::TryInto;
use sodalite::SIGN_LEN;
use sp_std::{prelude::*, vec::Vec};

use crate::{
    keypair::SecretKey,
    network::Network,
    types::{
        AccountId, DecoratedSignature, Memo, MuxedAccount, Operation, PublicKey, TimeBounds,
        Transaction, TransactionEnvelope, TransactionExt, TransactionSignaturePayload,
        TransactionSignaturePayloadTaggedTransaction, TransactionV0Ext, TransactionV1Envelope,
    },
    utils::{
        base64,
        sha256::{sha256, BinarySha256Hash},
    },
    xdr::{
        compound_types::{LimitedVarArray, LimitedVarOpaque},
        xdr_codec::XdrCodec,
    },
    Error, BASE_FEE_STROOPS,
};

impl Transaction {
    pub fn new(
        source_account: &str,
        sequence_number: i64,
        fee: Option<u32>,
        time_bounds: Option<TimeBounds>,
        memo: Option<Memo>,
    ) -> Result<Self, Error> {
        let source_public_key = AccountId::from_encoding(source_account)?;

        let transaction = Transaction {
            source_account: MuxedAccount::KeyTypeEd25519(source_public_key.into_binary()),
            fee: fee.unwrap_or(BASE_FEE_STROOPS),
            seq_num: sequence_number,
            time_bounds,
            memo: memo.unwrap_or(Memo::MemoNone),
            operations: LimitedVarArray::new_empty(),
            ext: TransactionExt::V0,
        };

        Ok(transaction)
    }

    pub fn append_operation(&mut self, operation: Operation) -> Result<(), Error> {
        self.operations.push(operation)
    }

    pub fn into_transaction_envelope(self) -> TransactionEnvelope {
        TransactionEnvelope::EnvelopeTypeTx(TransactionV1Envelope {
            tx: self,
            signatures: LimitedVarArray::new_empty(),
        })
    }
}

impl TransactionEnvelope {
    fn get_signatures(&mut self) -> &mut LimitedVarArray<DecoratedSignature, 20> {
        match self {
            TransactionEnvelope::EnvelopeTypeTxV0(envelope) => &mut envelope.signatures,
            TransactionEnvelope::EnvelopeTypeTx(envelope) => &mut envelope.signatures,
            TransactionEnvelope::EnvelopeTypeTxFeeBump(envelope) => &mut envelope.signatures,
            _ => unreachable!("Invalid transaction envelope type"),
        }
    }

    /// Generate a base64 encoded signature
    ///
    /// Generate a signature for the `transaction_envelope`. Generate the signature
    /// for a network having the passphrase contained in `network`. The secret key for
    /// signing the envelope is provided by `keypair`.
    /// The signature is not appended to the transaction envelope.
    pub fn create_base64_signature(&self, network: &Network, keypair: &SecretKey) -> Vec<u8> {
        let transaction_hash = self.get_hash(network);
        let signature = keypair.create_signature(transaction_hash);
        base64::encode(signature)
    }

    /// Generate and add signatures to a transaction envelope
    ///
    /// Generate and add signatures to the `transaction_envelope`. The signature
    /// is generated for a network having the passphrase contained in `network`. Generate and add
    /// one signature for each keypair in `keypairs`.
    pub fn sign(&mut self, network: &Network, keypairs: Vec<&SecretKey>) -> Result<(), Error> {
        let transaction_hash = self.get_hash(network);

        let signatures = self.get_signatures();

        for keypair in keypairs.iter() {
            let signature = keypair.create_signature(transaction_hash);
            let hint = keypair.get_public().get_signature_hint();

            signatures
                .push(DecoratedSignature {
                    hint,
                    signature: LimitedVarOpaque::new(Vec::from(signature)).unwrap(),
                })
                .map_err(|_| Error::TooManySignatures)?;
        }

        Ok(())
    }

    /// Add a base64 encoded signature to a transaction envelope
    ///
    /// Add a previously generated base64 encoded signature to the `transaction_envelope`.
    /// This function verifies whether the signature is valid given the passphrase contained in `network`
    /// and the `public_key`.
    pub fn add_base64_signature<T: AsRef<[u8]>>(
        &mut self,
        network: &Network,
        base64_signature: T,
        public_key: &PublicKey,
    ) -> Result<(), Error> {
        let signature = match base64::decode(base64_signature) {
            Err(err) => Err(Error::InvalidBase64Encoding(err))?,
            Ok(signature) => {
                if signature.len() != SIGN_LEN {
                    return Err(Error::InvalidSignatureLength {
                        found_length: signature.len(),
                        expected_length: SIGN_LEN,
                    });
                };
                signature
            }
        };

        let transaction_hash = self.get_hash(network);
        if !public_key.verify_signature(transaction_hash, signature[..].try_into().unwrap()) {
            return Err(Error::PublicKeyCantVerify);
        }

        let signatures = self.get_signatures();

        signatures
            .push(DecoratedSignature {
                hint: public_key.get_signature_hint(),
                signature: LimitedVarOpaque::new(signature).unwrap(),
            })
            .map_err(|_| Error::TooManySignatures)?;

        Ok(())
    }

    fn get_hash(&self, network: &Network) -> BinarySha256Hash {
        let network_id = network.get_id().clone();

        let tagged_transaction = match self {
            TransactionEnvelope::EnvelopeTypeTxV0(envelope) => {
                let transaction = Transaction {
                    source_account: MuxedAccount::KeyTypeEd25519(
                        envelope.tx.source_account_ed25519,
                    ),
                    fee: envelope.tx.fee,
                    seq_num: envelope.tx.seq_num,
                    time_bounds: envelope.tx.time_bounds.clone(),
                    memo: envelope.tx.memo.clone(),
                    operations: envelope.tx.operations.clone(),
                    ext: match envelope.tx.ext {
                        TransactionV0Ext::V0 => TransactionExt::V0,
                        TransactionV0Ext::Default(default) => TransactionExt::Default(default),
                    },
                };
                TransactionSignaturePayloadTaggedTransaction::EnvelopeTypeTx(transaction)
            }

            TransactionEnvelope::EnvelopeTypeTx(envelope) => {
                TransactionSignaturePayloadTaggedTransaction::EnvelopeTypeTx(envelope.tx.clone())
            }

            TransactionEnvelope::EnvelopeTypeTxFeeBump(envelope) => {
                TransactionSignaturePayloadTaggedTransaction::EnvelopeTypeTxFeeBump(
                    envelope.tx.clone(),
                )
            }

            _ => unimplemented!("This type of transaction envelope is not supported"),
        };

        let signature_payload = TransactionSignaturePayload {
            network_id,
            tagged_transaction,
        };

        sha256(signature_payload.to_xdr())
    }
}

#[cfg(test)]
mod tests {
    use sp_std::{prelude::*, vec::Vec};

    use crate::{
        types::{
            Asset, AssetAlphaNum4, ManageSellOfferOp, Memo, MuxedAccount, Operation, OperationBody,
            PaymentOp, Price, PublicKey, TimeBounds, Transaction, TransactionEnvelope,
            TransactionExt, TransactionMeta, TransactionV1Envelope, Uint256,
        },
        xdr::compound_types::LimitedVarArray,
        XdrCodec,
    };

    use crate::{keypair::SecretKey, network::TEST_NETWORK};

    const ENVELOPE: &[u8; 408] = b"AAAAAgAAAAC9xFYU1gQJeH4apEfzJkMCsW5DL4GEWRpyVjQHOlWVzgAAAZA\
        CGsQoAAQytgAAAAAAAAAAAAAAAgAAAAAAAAADAAAAAVhMUEcAAAAAxxJMrxQQOx9raxDm3\
        lINsLvksi7tj1BCQXzWTtqigbgAAAAAAAAAAAbK5N8CprKDAExLQAAAAAAAAAAAAAAAAAA\
        AAAMAAAAAAAAAAVhMUEcAAAAAxxJMrxQQOx9raxDm3lINsLvksi7tj1BCQXzWTtqigbgAA\
        AAAlV2+xQAEaBMAJiWgAAAAAAAAAAAAAAAAAAAAATpVlc4AAABAaX11e1dGcDkXrFT5s3Q\
        N6x3v4kQqJ/1VIjqO00y6OStd70/aYiXR35e4289RvmBTudJ5Q05PaRsD8p1qa17VDQ==";

    const META: &[u8; 2060] = b"AAAAAgAAAAIAAAADAiOf2gAAAAAAAAAAvcRWFNYECXh+GqRH8yZDArFuQy+Bh\
        FkaclY0BzpVlc4AAAABMLFdwgIaxCgABDK1AAAAAQAAAAAAAAAAAAAAAAEAAAAAAAAAAAAA\
        AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAECI5/aAAAAAAAAAAC9xFYU1gQJeH4apEf\
        zJkMCsW5DL4GEWRpyVjQHOlWVzgAAAAEwsV3CAhrEKAAEMrYAAAABAAAAAAAAAAAAAAAAAQ\
        AAAAAAAAAAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAgAAAAUAAAADAiOf2gAAA\
        AAAAAAAvcRWFNYECXh+GqRH8yZDArFuQy+BhFkaclY0BzpVlc4AAAABMLFdwgIaxCgABDK2\
        AAAAAQAAAAAAAAAAAAAAAAEAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\
        AAAECI5/aAAAAAAAAAAC9xFYU1gQJeH4apEfzJkMCsW5DL4GEWRpyVjQHOlWVzgAAAAEwsV\
        3CAhrEKAAEMrYAAAACAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAABAAAAADxs5AUAAAAAAAAAA\
        AAAAAAAAAAAAAAAAAIjn9oAAAACAAAAAL3EVhTWBAl4fhqkR/MmQwKxbkMvgYRZGnJWNAc6\
        VZXOAAAAACWxxV0AAAABWExQRwAAAADHEkyvFBA7H2trEObeUg2wu+SyLu2PUEJBfNZO2qK\
        BuAAAAAAAAAAABsrk3wKmsoMATEtAAAAAAAAAAAAAAAAAAAAAAwIjn9gAAAABAAAAAL3EVh\
        TWBAl4fhqkR/MmQwKxbkMvgYRZGnJWNAc6VZXOAAAAAVhMUEcAAAAAxxJMrxQQOx9raxDm3\
        lINsLvksi7tj1BCQXzWTtqigbgAAAAADaUL/n//////////AAAAAQAAAAEAAAAAAAAAAAAA\
        AAAAAAAAAAAAAAAAAAAAAAABAiOf2gAAAAEAAAAAvcRWFNYECXh+GqRH8yZDArFuQy+BhFk\
        aclY0BzpVlc4AAAABWExQRwAAAADHEkyvFBA7H2trEObeUg2wu+SyLu2PUEJBfNZO2qKBuA\
        AAAAANpQv+f/////////8AAAABAAAAAQAAAAAAAAAAAAAAAAbK5N8AAAAAAAAAAAAAAAUAA\
        AADAiOf2gAAAAAAAAAAvcRWFNYECXh+GqRH8yZDArFuQy+BhFkaclY0BzpVlc4AAAABMLFd\
        wgIaxCgABDK2AAAAAgAAAAAAAAAAAAAAAAEAAAAAAAAAAAAAAQAAAAA8bOQFAAAAAAAAAAA\
        AAAAAAAAAAAAAAAECI5/aAAAAAAAAAAC9xFYU1gQJeH4apEfzJkMCsW5DL4GEWRpyVjQHOl\
        WVzgAAAAEwsV3CAhrEKAAEMrYAAAADAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAABAAAAADxs5\
        AUAAAAAlV2+wgAAAAAAAAAAAAAAAAIjn9oAAAACAAAAAL3EVhTWBAl4fhqkR/MmQwKxbkMv\
        gYRZGnJWNAc6VZXOAAAAACWxxV4AAAAAAAAAAVhMUEcAAAAAxxJMrxQQOx9raxDm3lINsLv\
        ksi7tj1BCQXzWTtqigbgAAAAAlV2+wgAEaBMAJiWgAAAAAAAAAAAAAAAAAAAAAwIjn9oAAA\
        ABAAAAAL3EVhTWBAl4fhqkR/MmQwKxbkMvgYRZGnJWNAc6VZXOAAAAAVhMUEcAAAAAxxJMr\
        xQQOx9raxDm3lINsLvksi7tj1BCQXzWTtqigbgAAAAADaUL/n//////////AAAAAQAAAAEA\
        AAAAAAAAAAAAAAAGyuTfAAAAAAAAAAAAAAABAiOf2gAAAAEAAAAAvcRWFNYECXh+GqRH8yZ\
        DArFuQy+BhFkaclY0BzpVlc4AAAABWExQRwAAAADHEkyvFBA7H2trEObeUg2wu+SyLu2PUE\
        JBfNZO2qKBuAAAAAANpQv+f/////////8AAAABAAAAAQAAAAARQQaGAAAAAAbK5N8AAAAAA\
        AAAAAAAAAA=";

    fn binary_public(public: &str) -> Uint256 {
        PublicKey::from_encoding(public).unwrap().into_binary()
    }

    #[test]
    fn xdr_encode_decode_transaction_envelope() {
        let envelope = TransactionEnvelope::from_base64_xdr(ENVELOPE).unwrap();
        assert_eq!(ENVELOPE, &envelope.to_base64_xdr()[..]);

        let meta = TransactionMeta::from_base64_xdr(META).unwrap();
        assert_eq!(META, &meta.to_base64_xdr()[..]);
    }

    #[test]
    fn decode_complex_transaction() {
        let envelope = "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";

        let envelope = TransactionEnvelope::from_base64_xdr(envelope);
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
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GCQTGZQQ5G4PTM2GL7CDIFKUBIPEC52BROAQIAPW53XBRJVN6ZJVTG6V",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"LTC\0".clone(),
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
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
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GCQTGZQQ5G4PTM2GL7CDIFKUBIPEC52BROAQIAPW53XBRJVN6ZJVTG6V",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"LTC\0".clone(),
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
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
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GCQTGZQQ5G4PTM2GL7CDIFKUBIPEC52BROAQIAPW53XBRJVN6ZJVTG6V",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"LTC\0".clone(),
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
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
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GCQTGZQQ5G4PTM2GL7CDIFKUBIPEC52BROAQIAPW53XBRJVN6ZJVTG6V",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"LTC\0".clone(),
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
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
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GCQTGZQQ5G4PTM2GL7CDIFKUBIPEC52BROAQIAPW53XBRJVN6ZJVTG6V",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"LTC\0".clone(),
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
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
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
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
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
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
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
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
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
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
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
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
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
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
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
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
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
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
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
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
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
                                "GC5LOR3BK6KIOK7GKAUD5EGHQCMFOGHJTC7I3ELB66PTDFXORC2VM5LP",
                            )),
                        }),
                        buying: Asset::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                            asset_code: b"USDT".clone(),
                            issuer: PublicKey::PublicKeyTypeEd25519(binary_public(
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
        let keypair = SecretKey::from_encoding(secret);
        assert!(keypair.is_ok());
        let keypair = keypair.unwrap();

        let dest_public =
            PublicKey::from_encoding("GDMTKCJQ322RDTGOBLIPVEUCO3EIEJLXDV4JTWLXU6AFOYTMSJ45WZY5")
                .unwrap();

        let mut transaction_envelope = TransactionEnvelope::EnvelopeTypeTx(TransactionV1Envelope {
            tx: Transaction {
                source_account: MuxedAccount::KeyTypeEd25519(
                    keypair.get_public().clone().into_binary(),
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
                        destination: MuxedAccount::KeyTypeEd25519(dest_public.into_binary()),
                        asset: Asset::AssetTypeNative,
                        amount: 10_000_000,
                    }),
                }])
                .unwrap(),
                ext: TransactionExt::V0,
            },
            signatures: LimitedVarArray::new(Vec::new()).unwrap(),
        });

        let expected_xdr = b"AAAAAgAAAABRVWJF9F/Kd+p+e65fn2mnDGH\
        5BnlL9yXAMBTaJbnUcQAAJxAAADYZAAAAAQAAAAEAAAAAAAAAAAAAAAAAAAA\
        AAAAAAAAAAAEAAAAAAAAAAQAAAADZNQkw3rURzM4K0PqSgnbIgiV3HXiZ2Xe\
        ngFdibJJ52wAAAAAAAAAAAJiWgAAAAAAAAAAA";
        assert_eq!(
            transaction_envelope.to_base64_xdr().as_slice(),
            &expected_xdr[..]
        );

        let signing_result = transaction_envelope.sign(&TEST_NETWORK, vec![&keypair]);
        assert!(signing_result.is_ok());

        let expected_signed_xdr = b"AAAAAgAAAABRVWJF9F/Kd+p+e65\
        fn2mnDGH5BnlL9yXAMBTaJbnUcQAAJxAAADYZAAAAAQAAAAEAAAAAAAAAAA\
        AAAAAAAAAAAAAAAAAAAAEAAAAAAAAAAQAAAADZNQkw3rURzM4K0PqSgnbIg\
        iV3HXiZ2XengFdibJJ52wAAAAAAAAAAAJiWgAAAAAAAAAABJbnUcQAAAEAv\
        CLQxbuE/zeBYq5Q/17d1hvcQME5uHUJ9SE8L8E/PQHa00jfGpFrtsG+XQV0\
        DI0AnnqQhBhHKl1l5LNpIoxIA";

        assert_eq!(
            transaction_envelope.to_base64_xdr().as_slice(),
            &expected_signed_xdr[..]
        );
    }
}
