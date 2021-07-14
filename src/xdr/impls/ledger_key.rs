use crate::{
    compound_types::LimitedString,
    types::{
        LedgerKeyAccount, LedgerKeyClaimableBalance, LedgerKeyData, LedgerKeyLiquidityPool,
        LedgerKeyOffer, LedgerKeyTrustLine,
    },
    Asset, IntoAccountId, IntoClaimbleBalanceId, IntoHash, LedgerKey, StellarSdkError,
};

impl LedgerKey {
    pub fn from_account_id<T: IntoAccountId>(account_id: T) -> Result<Self, StellarSdkError> {
        let account_id = account_id.into_account_id()?;
        Ok(Self::Account(LedgerKeyAccount { account_id }))
    }

    pub fn from_trustline<T: IntoAccountId>(
        account_id: T,
        asset: Asset,
    ) -> Result<Self, StellarSdkError> {
        let account_id = account_id.into_account_id()?;
        Ok(Self::Trustline(LedgerKeyTrustLine { account_id, asset }))
    }

    pub fn from_offer<T: IntoAccountId>(
        seller_id: T,
        offer_id: i64,
    ) -> Result<Self, StellarSdkError> {
        let seller_id = seller_id.into_account_id()?;
        Ok(Self::Offer(LedgerKeyOffer {
            seller_id,
            offer_id,
        }))
    }

    pub fn from_data<T: IntoAccountId, S: AsRef<[u8]>>(
        account_id: T,
        data_name: S,
    ) -> Result<Self, StellarSdkError> {
        let account_id = account_id.into_account_id()?;
        let data_name = LimitedString::new(data_name.as_ref().to_vec())?;
        Ok(Self::Data(LedgerKeyData {
            account_id,
            data_name,
        }))
    }

    pub fn from_claimable_balance_id<T: IntoClaimbleBalanceId>(
        balance_id: T,
    ) -> Result<Self, StellarSdkError> {
        let balance_id = balance_id.into_claimable_balance_id()?;
        Ok(Self::ClaimableBalance(LedgerKeyClaimableBalance {
            balance_id,
        }))
    }

    pub fn from_liquidity_pool_id<T: IntoHash>(
        liquidity_pool_id: T,
    ) -> Result<Self, StellarSdkError> {
        let liquidity_pool_id = liquidity_pool_id.into_hash()?;
        Ok(Self::LiquidityPool(LedgerKeyLiquidityPool {
            liquidity_pool_id,
        }))
    }
}
