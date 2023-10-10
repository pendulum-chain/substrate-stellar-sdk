use crate::{types::TransactionSet, Hash, IntoHash, StellarSdkError};

impl IntoHash for TransactionSet {
    fn into_hash(self) -> Result<Hash, StellarSdkError> {
        use crate::XdrCodec;
        use sha2::{Digest, Sha256};
        use sp_std::convert::TryInto;

        let mut hasher = Sha256::new();
        hasher.update(self.previous_ledger_hash);

        self.txes.get_vec().iter().for_each(|envlp| {
            hasher.update(envlp.to_xdr());
        });

        hasher
            .finalize()
            .as_slice()
            .try_into()
            .map_err(|_| StellarSdkError::InvalidHashConversion)
    }
}
