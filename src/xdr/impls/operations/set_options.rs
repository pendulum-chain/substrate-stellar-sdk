use core::convert::AsRef;

use crate::{
    compound_types::LimitedString,
    types::{OperationBody, SetOptionsOp},
    Error, IntoAccountId, IntoMuxedAccountId, Operation, Signer,
};

impl Operation {
    pub fn new_set_options<T: IntoAccountId, S: AsRef<[u8]>, U: IntoMuxedAccountId>(
        source_account: Option<U>,
        inflation_dest: Option<T>,
        clear_flags: Option<u32>,
        set_flags: Option<u32>,
        master_weight: Option<u8>,
        low_threshold: Option<u8>,
        med_threshold: Option<u8>,
        high_threshold: Option<u8>,
        home_domain: Option<S>,
        signer: Option<Signer>,
    ) -> Result<Operation, Error> {
        let source_account = source_account.map(<_>::into_muxed_account_id).transpose()?;

        let home_domain = match home_domain {
            Some(home_domain) => Some(LimitedString::new(home_domain.as_ref().to_vec())?),
            None => None,
        };

        let inflation_dest = match inflation_dest {
            Some(inflation_dest) => Some(inflation_dest.into_account_id()?),
            None => None,
        };

        Ok(Operation {
            source_account,
            body: OperationBody::SetOptions(SetOptionsOp {
                inflation_dest,
                clear_flags,
                set_flags,
                master_weight: master_weight.map(|weight| weight as u32),
                low_threshold: low_threshold.map(|weight| weight as u32),
                med_threshold: med_threshold.map(|weight| weight as u32),
                high_threshold: high_threshold.map(|weight| weight as u32),
                home_domain,
                signer,
            }),
        })
    }
}
