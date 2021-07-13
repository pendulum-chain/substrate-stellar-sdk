use crate::Error;
use core::convert::{AsRef, From, TryFrom, TryInto};
use sp_std::str;

pub const STROOPS_PER_LUMEN: i64 = 10_000_000;
pub struct LumenAmount(pub f64);
pub struct StroopAmount(pub i64);

pub trait AsAmount {
    fn as_stroop_amount(self, allow_zero: bool) -> Result<i64, Error>;
}

impl AsAmount for StroopAmount {
    fn as_stroop_amount(self, allow_zero: bool) -> Result<i64, Error> {
        if allow_zero {
            match self.0 < 0 {
                true => Err(Error::AmountNegative),
                false => Ok(self.0),
            }
        } else {
            match self.0 <= 0 {
                true => Err(Error::AmountNonPositive),
                false => Ok(self.0),
            }
        }
    }
}

impl AsAmount for LumenAmount {
    fn as_stroop_amount(self, allow_zero: bool) -> Result<i64, Error> {
        let stroop_amount: StroopAmount = self.try_into()?;
        stroop_amount.as_stroop_amount(allow_zero)
    }
}

impl TryFrom<LumenAmount> for StroopAmount {
    type Error = Error;

    fn try_from(value: LumenAmount) -> Result<Self, Self::Error> {
        let float_stroops = value.0 * STROOPS_PER_LUMEN as f64;
        if float_stroops > i64::MAX as f64 {
            return Err(Error::AmountOverflow);
        }

        Ok(StroopAmount(float_stroops as i64))
    }
}

impl From<StroopAmount> for LumenAmount {
    fn from(value: StroopAmount) -> Self {
        LumenAmount(value.0 as f64 / STROOPS_PER_LUMEN as f64)
    }
}

impl<T: AsRef<[u8]>> AsAmount for T {
    fn as_stroop_amount(self, allow_zero: bool) -> Result<i64, Error> {
        let string = self.as_ref();
        let seperator_position = string.iter().position(|char| *char == b'.');

        let (integer_part, decimals) = match seperator_position {
            Some(seperator_position) => {
                let decimals_length = string.len() - seperator_position - 1;
                if decimals_length > 7 {
                    return Err(Error::InvalidAmountString);
                }
                let mut decimals = [b'0'; 7];
                decimals[..decimals_length].copy_from_slice(&string[seperator_position + 1..]);

                (&string[..seperator_position], parse_integer(&decimals)?)
            }
            None => (&string[..], 0),
        };

        let integer_part = parse_integer(integer_part)?;

        let result = match integer_part.checked_mul(STROOPS_PER_LUMEN) {
            Some(result) => result,
            None => return Err(Error::AmountOverflow),
        };

        let result = match result.checked_add(decimals) {
            Some(result) => result,
            None => return Err(Error::AmountOverflow),
        };

        if result == 0 && !allow_zero {
            return Err(Error::AmountNonPositive);
        }

        Ok(result)
    }
}

fn parse_integer(slice: &[u8]) -> Result<i64, Error> {
    if !slice.iter().all(|char| (*char as char).is_ascii_digit()) {
        return Err(Error::InvalidAmountString);
    }
    let slice = str::from_utf8(slice).unwrap();
    slice.parse().map_err(|_| Error::InvalidAmountString)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_lumen_string() {
        assert_eq!("23".as_stroop_amount(true), Ok(230_000_000));
        assert_eq!(
            "922337203685".as_stroop_amount(true),
            Ok(9223372036850_000_000)
        );
        assert_eq!(
            "922337203686".as_stroop_amount(true),
            Err(Error::AmountOverflow)
        );

        assert_eq!("0.23".as_stroop_amount(true), Ok(2_300_000));
        assert_eq!("0.232442".as_stroop_amount(true), Ok(2_324_420));
        assert_eq!("14.2324426".as_stroop_amount(true), Ok(142_324_426));
        assert_eq!(
            "14.23244267".as_stroop_amount(true),
            Err(Error::InvalidAmountString)
        );

        assert_eq!("420.".as_stroop_amount(true), Ok(4200_000_000));

        // maximal value allowed in Stellar (max value that fits in a i64)
        assert_eq!(
            "922337203685.4775807".as_stroop_amount(true),
            Ok(9223372036854775807)
        );
        assert_eq!("922337203685.4775807".as_stroop_amount(true), Ok(i64::MAX));

        // one more stroop and it overflows
        assert_eq!(
            "922337203685.4775808".as_stroop_amount(true),
            Err(Error::AmountOverflow)
        );

        assert_eq!(".".as_stroop_amount(true), Err(Error::InvalidAmountString));
        assert_eq!("".as_stroop_amount(true), Err(Error::InvalidAmountString));

        assert_eq!(
            "243. 34".as_stroop_amount(true),
            Err(Error::InvalidAmountString)
        );
        assert_eq!(
            "243.+34".as_stroop_amount(true),
            Err(Error::InvalidAmountString)
        );
        assert_eq!(
            "+243.34".as_stroop_amount(true),
            Err(Error::InvalidAmountString)
        );
        assert_eq!(
            "243.34x".as_stroop_amount(true),
            Err(Error::InvalidAmountString)
        );
        assert_eq!(
            "24?.34x".as_stroop_amount(true),
            Err(Error::InvalidAmountString)
        );
    }
}
