use core::convert::AsRef;

use crate::{
    types::{AssetAlphaNum12, AssetAlphaNum4},
    Asset, AssetCode, IntoAccountId, StellarSdkError,
};

impl Asset {
    pub fn native() -> Self {
        Asset::AssetTypeNative
    }

    pub fn from_asset_code<T: AsRef<[u8]>, S: IntoAccountId>(
        asset_code: T,
        issuer: S,
    ) -> Result<Self, StellarSdkError> {
        let asset_code = AssetCode::new(asset_code)?;

        match asset_code {
            AssetCode::AssetTypeCreditAlphanum4(asset_code) => {
                Ok(Self::AssetTypeCreditAlphanum4(AssetAlphaNum4 {
                    asset_code,
                    issuer: issuer.into_account_id()?,
                }))
            }
            AssetCode::AssetTypeCreditAlphanum12(asset_code) => {
                Ok(Self::AssetTypeCreditAlphanum12(AssetAlphaNum12 {
                    asset_code,
                    issuer: issuer.into_account_id()?,
                }))
            }
            AssetCode::Default(_) => unreachable!(),
        }
    }
}
