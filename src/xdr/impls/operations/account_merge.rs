use crate::{types::OperationBody, MuxedAccount, Operation};

impl Operation {
    pub fn new_account_merge(
        source_account: Option<MuxedAccount>,
        destination_account: MuxedAccount,
    ) -> Operation {
        Operation {
            source_account,
            body: OperationBody::AccountMerge(destination_account),
        }
    }
}
