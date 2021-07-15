use crate::{IntoMuxedAccountId, Operation, StellarSdkError};

pub mod account_merge;
pub mod allow_trust;
pub mod begin_sponsoring_future_reserves;
pub mod bump_sequence;
pub mod change_trust;
pub mod claim_claimable_balance;
pub mod clawback;
pub mod clawback_claimable_balance;
pub mod create_account;
pub mod create_claimable_balance;
pub mod create_passive_sell_offer;
pub mod end_sponsoring_future_reserves;
pub mod inflation;
pub mod manage_buy_offer;
pub mod manage_data;
pub mod manage_sell_offer;
pub mod path_payment_strict_receive;
pub mod path_payment_strict_send;
pub mod payment;
pub mod revoke_sponsorship;
pub mod set_options;
pub mod set_trust_line_flags;

impl Operation {
    pub fn set_source_account<T: IntoMuxedAccountId>(
        mut self,
        source_account: T,
    ) -> Result<Self, StellarSdkError> {
        self.source_account = Some(source_account.into_muxed_account_id()?);
        Ok(self)
    }
}
