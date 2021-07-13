use num_rational::Ratio;

use crate::{StellarSdkError, Price};

impl Price {
    pub fn from_fraction(denominator: i32, numerator: i32) -> Result<Price, StellarSdkError> {
        if denominator <= 0 || numerator <= 0 {
            return Err(StellarSdkError::InvalidPrice);
        }

        Ok(Price {
            d: denominator,
            n: numerator,
        })
    }

    pub fn from_float(price: f64) -> Result<Price, StellarSdkError> {
        if price <= 0.0 {
            return Err(StellarSdkError::InvalidPrice);
        }

        Ratio::<i32>::approximate_float(price)
            .ok_or(StellarSdkError::NotApproximableAsFraction)
            .map(|price| Price {
                d: *price.denom(),
                n: *price.numer(),
            })
    }
}
