use crate::{
    compound_types::LimitedString,
    keypair::AsPublicKey,
    types::{
        ClaimableBalanceId, LedgerKeyAccount, LedgerKeyClaimableBalance, LedgerKeyData,
        LedgerKeyLiquidityPool, LedgerKeyOffer, LedgerKeyTrustLine,
    },
    AsHash, Asset, Error, LedgerKey,
};

impl LedgerKey {
    pub fn from_account_id<T: AsPublicKey>(account_id: T) -> Result<Self, Error> {
        let account_id = account_id.as_public_key()?;
        Ok(Self::Account(LedgerKeyAccount { account_id }))
    }

    pub fn from_trustline<T: AsPublicKey>(account_id: T, asset: Asset) -> Result<Self, Error> {
        let account_id = account_id.as_public_key()?;
        Ok(Self::Trustline(LedgerKeyTrustLine { account_id, asset }))
    }

    pub fn from_offer<T: AsPublicKey>(seller_id: T, offer_id: i64) -> Result<Self, Error> {
        let seller_id = seller_id.as_public_key()?;
        Ok(Self::Offer(LedgerKeyOffer {
            seller_id,
            offer_id,
        }))
    }

    pub fn from_data<T: AsPublicKey>(account_id: T, data_name: Vec<u8>) -> Result<Self, Error> {
        let account_id = account_id.as_public_key()?;
        let data_name = LimitedString::new(data_name)?;
        Ok(Self::Data(LedgerKeyData {
            account_id,
            data_name,
        }))
    }

    pub fn from_claimable_balance_id<T: AsHash>(balance_id: T) -> Result<Self, Error> {
        let balance_id = balance_id.as_hash()?;
        Ok(Self::ClaimableBalance(LedgerKeyClaimableBalance {
            balance_id: ClaimableBalanceId::ClaimableBalanceIdTypeV0(balance_id),
        }))
    }

    pub fn from_liquidity_pool_id<T: AsHash>(liquidity_pool_id: T) -> Result<Self, Error> {
        let liquidity_pool_id = liquidity_pool_id.as_hash()?;
        Ok(Self::LiquidityPool(LedgerKeyLiquidityPool {
            liquidity_pool_id,
        }))
    }
}
