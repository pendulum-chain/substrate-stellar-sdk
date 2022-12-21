use core::num::{ParseFloatError, ParseIntError};
use sp_io::offchain::timestamp;
use sp_runtime::offchain::{
    http::{Error, Method, Request},
    Duration, HttpError,
};
use sp_std::{str, vec, vec::Vec};

use core::convert::TryInto;

use crate::{AccountId, IntoAccountId, StellarSdkError};

use super::{
    api_response_types::FeeStats, json_response_types, Horizon, HTTP_HEADER_CLIENT_NAME, HTTP_HEADER_CLIENT_VERSION,
};

impl From<ParseIntError> for FetchError {
    fn from(error: ParseIntError) -> Self {
        FetchError::ParseIntError(error)
    }
}

impl From<ParseFloatError> for FetchError {
    fn from(error: ParseFloatError) -> Self {
        FetchError::ParseFloatError(error)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FetchError {
    DeadlineReached,
    IoError,
    Invalid,
    Unknown,
    UnexpectedResponseStatus { status: u16, body: Vec<u8> },
    JsonParseError,
    InvalidSequenceNumber,
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    AccountRequiredMemo(AccountId),
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

impl From<FetchError> for StellarSdkError {
    fn from(error: FetchError) -> Self {
        StellarSdkError::FetchError(error)
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
    pub fn request(&self, path: Vec<&[u8]>, method: Method, timeout_milliseconds: u64) -> Result<Vec<u8>, FetchError> {
        let mut url = self.base_url.clone();
        for path_segment in path {
            url.extend_from_slice(path_segment);
        }

        let request = Request::<Vec<&'static [u8]>>::new(str::from_utf8(&url).unwrap()).method(method);
        let timeout = timestamp().add(Duration::from_millis(timeout_milliseconds));
        let pending = request
            .add_header("X-Client-Name", HTTP_HEADER_CLIENT_NAME)
            .add_header("X-Client-Version", HTTP_HEADER_CLIENT_VERSION)
            .deadline(timeout)
            .send()?;

        let response = pending.try_wait(timeout).map_err(|_| FetchError::DeadlineReached)?;
        let response = response?;

        if response.code != 200 {
            return Err(FetchError::UnexpectedResponseStatus { status: response.code, body: response.body().collect() })
        }

        Ok(response.body().collect())
    }

    /// Fetch the sequence number of an account
    ///
    /// The sequence number is defined to be of type [i64](https://github.com/stellar/stellar-core/blob/master/src/xdr/Stellar-ledger-entries.x)
    pub fn fetch_fee_stats(&self, timeout_milliseconds: u64) -> Result<FeeStats, FetchError> {
        let json = self.request(vec![b"/fee_stats"], Method::Get, timeout_milliseconds)?;

        let response: json_response_types::FeeStats = serde_json::from_slice(&json)?;

        response.try_into()
    }

    /// Fetch the sequence number of an account
    ///
    /// The sequence number is defined to be of type [i64](https://github.com/stellar/stellar-core/blob/master/src/xdr/Stellar-ledger-entries.x)
    pub fn fetch_account<T: IntoAccountId>(
        &self,
        account_id: T,
        timeout_milliseconds: u64,
    ) -> Result<json_response_types::AccountResponse, FetchError> {
        let json = self.request(
            vec![b"/accounts/", account_id.into_encoding().as_slice()],
            Method::Get,
            timeout_milliseconds,
        )?;

        let account_response: json_response_types::AccountResponse = serde_json::from_slice(&json)?;

        Ok(account_response)
    }

    /// Fetch the sequence number of an account
    ///
    /// The sequence number is defined to be of type [i64](https://github.com/stellar/stellar-core/blob/master/src/xdr/Stellar-ledger-entries.x)
    pub fn fetch_next_sequence_number<T: IntoAccountId>(
        &self,
        account_id: T,
        timeout_milliseconds: u64,
    ) -> Result<i64, FetchError> {
        let account_response = self.fetch_account(account_id, timeout_milliseconds)?;

        let sequence_number: i64 = match account_response.sequence.parse() {
            Ok(n) => n,
            Err(_) => return Err(FetchError::InvalidSequenceNumber),
        };
        let next_sequence_number = sequence_number + 1;
        Ok(next_sequence_number)
    }
}
