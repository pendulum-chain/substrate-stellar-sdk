use super::{
    compound_types::{ExceedsMaximumLengthError, LimitedString},
    types::Memo,
};

impl Memo {
    pub fn from_text_memo(text: &str) -> Result<Self, ExceedsMaximumLengthError> {
        Ok(Self::MemoText(LimitedString::new(
            text.as_bytes().to_vec(),
        )?))
    }

    pub fn from_id_memo(id: u64) -> Self {
        Self::MemoId(id)
    }

    pub fn from_hash_memo(hash: [u8; 32]) -> Self {
        Self::MemoHash(hash)
    }

    pub fn from_return_hash_memo(return_hash: [u8; 32]) -> Self {
        Self::MemoReturn(return_hash)
    }
}
