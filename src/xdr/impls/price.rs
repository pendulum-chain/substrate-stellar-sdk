use num_rational::Ratio;

use crate::{Error, Price};

impl Price {
    pub fn from_fraction(denominator: i32, numerator: i32) -> Result<Price, Error> {
        if denominator <= 0 || numerator <= 0 {
            return Err(Error::InvalidPrice);
        }

        Ok(Price {
            d: denominator,
            n: numerator,
        })
    }

    pub fn from_float(price: f64) -> Result<Price, Error> {
        if price <= 0.0 {
            return Err(Error::InvalidPrice);
        }

        Ratio::<i32>::approximate_float(price)
            .ok_or(Error::NotApproximableAsFraction)
            .map(|price| Price {
                d: *price.denom(),
                n: *price.numer(),
            })
    }
}
