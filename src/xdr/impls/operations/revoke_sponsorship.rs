use crate::{
    types::{OperationBody, RevokeSponsorshipOp, RevokeSponsorshipOpSigner},
    IntoMuxedAccountId, IntoPublicKey, LedgerKey, Operation, SignerKey, StellarSdkError,
};

impl Operation {
    pub fn new_revoke_sponsorship_ledger_entry<T: IntoMuxedAccountId>(
        source_account: Option<T>,
        ledger_key: LedgerKey,
    ) -> Result<Operation, StellarSdkError> {
        let source_account = match source_account {
            Some(source_account) => Some(source_account.into_muxed_account_id()?),
            None => None,
        };

        Ok(Operation {
            source_account,
            body: OperationBody::RevokeSponsorship(
                RevokeSponsorshipOp::RevokeSponsorshipLedgerEntry(ledger_key),
            ),
        })
    }

    pub fn new_revoke_sponsorship_signer<T: IntoPublicKey, S: IntoMuxedAccountId>(
        source_account: Option<S>,
        account_id: T,
        signer_key: SignerKey,
    ) -> Result<Operation, StellarSdkError> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        Ok(Operation {
            source_account,
            body: OperationBody::RevokeSponsorship(RevokeSponsorshipOp::RevokeSponsorshipSigner(
                RevokeSponsorshipOpSigner {
                    account_id: account_id.into_public_key()?,
                    signer_key,
                },
            )),
        })
    }
}
