use crate::{types::ClaimantV0, ClaimPredicate, Claimant, IntoAccountId, StellarSdkError};

impl Claimant {
    pub fn new<T: IntoAccountId>(destination: T, predicate: ClaimPredicate) -> Result<Self, StellarSdkError> {
        Ok(Claimant::ClaimantTypeV0(ClaimantV0 { destination: destination.into_account_id()?, predicate }))
    }
}
