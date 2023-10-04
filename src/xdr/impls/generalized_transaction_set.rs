use crate::{
    compound_types::UnlimitedVarArray,
    types::{GeneralizedTransactionSet, TransactionPhase, TxSetComponent},
    Hash, TransactionEnvelope, XdrCodec,
};

impl GeneralizedTransactionSet {
    #[cfg(feature = "std")]
    pub fn to_base64_encoded_xdr_string(&self) -> Result<String, std::string::FromUtf8Error> {
        let base_64 = self.clone().to_base64_xdr();
        String::from_utf8(base_64)
    }

    pub fn previous_ledger_hash(&self) -> Option<Hash> {
        let GeneralizedTransactionSet::V1(txset_v1) = self else {
            return None;
        };
        Some(txset_v1.previous_ledger_hash)
    }

    pub fn txes(&self) -> Option<UnlimitedVarArray<TransactionEnvelope>> {
        let GeneralizedTransactionSet::V1(txset_v1) = self else {
            return None;
        };

        let mut final_txes = UnlimitedVarArray::new_empty();

        for phase in txset_v1.phases.get_vec() {
            let TransactionPhase::V0(txsets_comp) = phase else {
                return None;
            };

            for TxSetComponent::TxsetCompTxsMaybeDiscountedFee(discounted_txs_set) in txsets_comp.get_vec() {
                if final_txes.len() == 0 {
                    final_txes = discounted_txs_set.txes.clone();
                } else {
                    for tx in discounted_txs_set.txes.get_vec() {
                        if let Err(_) = final_txes.push(tx.clone()) {
                            return None
                        }
                    }
                }
            }
        }

        Some(final_txes)
    }
}

#[cfg(test)]
mod tests {
    use crate::{types::GeneralizedTransactionSet, Hash, XdrCodec};

    const GENERALIZED_TX_SET: &str = "AAAAAVUcNbdctOS+OftZngTJY07YUUqb4P1I/owUmgdMuzscAAAAAgAAAAAAAAABAAAAAAAAAAEAAAAAAAAAZAAAAAEAAAACAAAAAMgOELhl9VFf5x0pG1aY8Mm/QQcnigdQ9MgWM1F8c6HSAAAAZAAZMt8AADrRAAAAAQAAAAAAAAAAAAAAAGUb2+8AAAABAAAAG3RzOjIwMjMtMTAtMDNUMDk6MTU6MDEuNTkyWgAAAAABAAAAAAAAAAEAAAAAf4MDV2AZH1oB1nouL9LSGUHGGafzcb48GXQyWFd9zswAAAAAAAAAAACYloAAAAAAAAAAAXxzodIAAABAEq3w/8HQ6kjqooVJPjg1TquL2pMOT+P9P7a3HpdqUYyFyJ8F32igbhIu3jvIJkafhDTosuL/rid2BxmScxhfDwAAAAAAAAAA";

    fn example() -> GeneralizedTransactionSet {
        let to_vec_u8 = GENERALIZED_TX_SET.as_bytes().to_vec();
        GeneralizedTransactionSet::from_base64_xdr(to_vec_u8).expect("should return a GeneralizedTransactionSet")
    }

    #[test]
    fn generalized_transaction_set_to_string() {
        let gen_txset_as_str = example()
            .to_base64_encoded_xdr_string()
            .expect("should be able to convert to string.");

        assert_eq!(gen_txset_as_str, GENERALIZED_TX_SET);
    }

    #[test]
    fn generalized_transaction_set_returns_prev_ledger_hash() {
        let expected_hash = "VRw1t1y05L45+1meBMljTthRSpvg/Uj+jBSaB0y7Oxw=".as_bytes();
        let expected_hash = Hash::from_base64_xdr(expected_hash).expect("return hash");

        let gen_tx_set = example();
        let actual_hash = gen_tx_set.previous_ledger_hash().expect("should return a previous ledger hash");

        assert_eq!(actual_hash, expected_hash);
    }

    #[test]
    fn generalized_transaction_set_returns_txset() {
        let gen_tx_set = example();
        let txes = gen_tx_set.txes().expect("should return an array of tx envelopes");

        assert!(!txes.get_vec().is_empty());
    }
}
