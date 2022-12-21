use super::{fetch::FetchError, json_response_types};
use core::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
pub struct FeeStats {
    pub last_ledger: u32,
    pub last_ledger_base_fee: i64,
    pub ledger_capacity_usage: f64,
    pub fee_charged: FeeDistribution,
    pub max_fee: FeeDistribution,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FeeDistribution {
    pub max: i64,
    pub min: i64,
    pub mode: i64,
    pub p10: i64,
    pub p20: i64,
    pub p30: i64,
    pub p40: i64,
    pub p50: i64,
    pub p60: i64,
    pub p70: i64,
    pub p80: i64,
    pub p90: i64,
    pub p95: i64,
    pub p99: i64,
}

impl TryFrom<json_response_types::FeeDistribution> for FeeDistribution {
    type Error = FetchError;

    fn try_from(fee_distribution: json_response_types::FeeDistribution) -> Result<Self, Self::Error> {
        Ok(FeeDistribution {
            max: fee_distribution.max.parse()?,
            min: fee_distribution.min.parse()?,
            mode: fee_distribution.mode.parse()?,
            p10: fee_distribution.p10.parse()?,
            p20: fee_distribution.p20.parse()?,
            p30: fee_distribution.p30.parse()?,
            p40: fee_distribution.p40.parse()?,
            p50: fee_distribution.p50.parse()?,
            p60: fee_distribution.p60.parse()?,
            p70: fee_distribution.p70.parse()?,
            p80: fee_distribution.p80.parse()?,
            p90: fee_distribution.p90.parse()?,
            p95: fee_distribution.p95.parse()?,
            p99: fee_distribution.p99.parse()?,
        })
    }
}

impl TryFrom<json_response_types::FeeStats> for FeeStats {
    type Error = FetchError;

    fn try_from(fee_stats: json_response_types::FeeStats) -> Result<Self, Self::Error> {
        Ok(FeeStats {
            last_ledger: fee_stats.last_ledger.parse()?,
            last_ledger_base_fee: fee_stats.last_ledger_base_fee.parse()?,
            ledger_capacity_usage: fee_stats.ledger_capacity_usage.parse()?,
            fee_charged: fee_stats.fee_charged.try_into()?,
            max_fee: fee_stats.max_fee.try_into()?,
        })
    }
}
