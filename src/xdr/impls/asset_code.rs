use core::convert::AsRef;

use crate::{AssetCode, StellarSdkError};

impl AssetCode {
    pub fn new<T: AsRef<[u8]>>(str: T) -> Result<Self, StellarSdkError> {
        let str = str.as_ref();
        if str.len() > 12 {
            return Err(StellarSdkError::AssetCodeTooLong)
        }

        if !str.iter().all(|char| {
            let char = char::from(*char);
            char.is_ascii_alphanumeric()
        }) {
            return Err(StellarSdkError::InvalidAssetCodeCharacter)
        }

        if str.len() <= 4 {
            let mut asset_code_array: [u8; 4] = [0; 4];
            asset_code_array[..str.len()].copy_from_slice(str);
            Ok(AssetCode::AssetTypeCreditAlphanum4(asset_code_array))
        } else {
            let mut asset_code_array: [u8; 12] = [0; 12];
            asset_code_array[..str.len()].copy_from_slice(str);
            Ok(AssetCode::AssetTypeCreditAlphanum12(asset_code_array))
        }
    }
}
