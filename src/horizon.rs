use sp_io::offchain::timestamp;
use sp_runtime::offchain::{http::Error, http::Request, Duration, HttpError};
use sp_std::{prelude::*, str, vec::Vec};

use crate::{horizon_types::AccountResponse, keypair::PublicKey};

pub struct Horizon {
    base_url: Vec<u8>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FetchError {
    DeadlineReached,
    IoError,
    Invalid,
    Unknown,
    UnexpectedResponseStatus { status: u16 },
    JsonParseError,
    InvalidSequenceNumber,
}

impl From<Error> for FetchError {
    fn from(error: Error) -> Self {
        match error {
            Error::DeadlineReached => FetchError::DeadlineReached,
            Error::IoError => FetchError::IoError,
            Error::Unknown => FetchError::Unknown,
        }
    }
}

impl From<HttpError> for FetchError {
    fn from(error: HttpError) -> Self {
        match error {
            HttpError::DeadlineReached => FetchError::DeadlineReached,
            HttpError::IoError => FetchError::IoError,
            HttpError::Invalid => FetchError::Invalid,
        }
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(_error: serde_json::Error) -> Self {
        FetchError::JsonParseError
    }
}

impl Horizon {
    pub fn new(base_url: &str) -> Horizon {
        Horizon {
            base_url: base_url.as_bytes().to_vec(),
        }
    }

    /// Fetch the sequence number of an account
    ///
    /// The sequence number is defined to be of type [i64](https://github.com/stellar/stellar-core/blob/master/src/xdr/Stellar-ledger-entries.x)
    pub fn fetch_sequence_number(
        &self,
        account_id: &PublicKey,
        timeout_milliseconds: u64,
    ) -> Result<i64, FetchError> {
        let mut url = self.base_url.clone();
        url.extend_from_slice(b"/accounts/");
        url.extend(account_id.to_encoding().iter());

        let request = Request::get(str::from_utf8(&url).unwrap());
        let timeout = timestamp().add(Duration::from_millis(timeout_milliseconds));
        let pending = request.deadline(timeout).send()?;

        let response = pending
            .try_wait(timeout)
            .map_err(|_| FetchError::DeadlineReached)?;
        let response = response?;

        if response.code != 200 {
            return Err(FetchError::UnexpectedResponseStatus {
                status: response.code,
            });
        }

        let json = response.body().collect::<Vec<u8>>();
        let account_response: AccountResponse = serde_json::from_slice(&json)?;

        let sequence_number: i64 = match account_response.sequence.parse() {
            Ok(n) => n,
            Err(_) => return Err(FetchError::InvalidSequenceNumber),
        };
        Ok(sequence_number)
    }
}
