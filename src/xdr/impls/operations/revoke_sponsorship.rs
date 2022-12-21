use crate::{
    types::{OperationBody, RevokeSponsorshipOp, RevokeSponsorshipOpSigner},
    IntoPublicKey, LedgerKey, Operation, SignerKey, StellarSdkError,
};

impl Operation {
    pub fn new_revoke_sponsorship_ledger_entry(ledger_key: LedgerKey) -> Result<Operation, StellarSdkError> {
        Ok(Operation {
            source_account: None,
            body: OperationBody::RevokeSponsorship(RevokeSponsorshipOp::RevokeSponsorshipLedgerEntry(ledger_key)),
        })
    }

    pub fn new_revoke_sponsorship_signer<T: IntoPublicKey>(
        account_id: T,
        signer_key: SignerKey,
    ) -> Result<Operation, StellarSdkError> {
        Ok(Operation {
            source_account: None,
            body: OperationBody::RevokeSponsorship(RevokeSponsorshipOp::RevokeSponsorshipSigner(
                RevokeSponsorshipOpSigner { account_id: account_id.into_public_key()?, signer_key },
            )),
        })
    }
}
