use crate::{types::TransactionSet, xdr::impls::hash::ComputeHash, Hash, XdrCodec};
use sha2::{Digest, Sha256};

#[cfg(feature = "std")]
impl ComputeHash for TransactionSet {
    fn compute_hash(&self) -> Option<Hash> {
        use std::convert::TryInto;

        let mut hasher = Sha256::new();
        hasher.update(self.previous_ledger_hash);

        self.txes.get_vec().iter().for_each(|envlp| {
            hasher.update(envlp.to_xdr());
        });

        hasher.finalize().as_slice().try_into().ok()
    }
}
