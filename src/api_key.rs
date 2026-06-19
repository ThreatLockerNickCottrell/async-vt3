use std::{
    borrow::Borrow,
    fmt::{Display, LowerHex},
    str::FromStr,
};

use hex::{FromHex, FromHexError, ToHex};

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ApiKey([u8; 32]);

impl ApiKey {
    #[inline]
    pub const fn new(api_key: [u8; 32]) -> Self {
        Self(api_key)
    }
}

impl AsRef<[u8]> for ApiKey {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Borrow<[u8]> for ApiKey {
    #[inline]
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}

impl FromHex for ApiKey {
    type Error = FromHexError;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        hex::decode(hex)?
            .as_array::<32>()
            .copied()
            .map(Self)
            .ok_or(FromHexError::InvalidStringLength)
    }
}

impl FromStr for ApiKey {
    type Err = <Self as FromHex>::Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ApiKey::from_hex(s)
    }
}

impl TryFrom<&str> for ApiKey {
    type Error = <Self as FromStr>::Err;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for ApiKey {
    type Error = <Self as FromStr>::Err;

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl LowerHex for ApiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.encode_hex::<String>().as_str())
    }
}

impl Display for ApiKey {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        LowerHex::fmt(&self, f)
    }
}

impl From<ApiKey> for String {
    #[inline]
    fn from(value: ApiKey) -> Self {
        value.to_string()
    }
}

impl From<ApiKey> for [u8; 32] {
    #[inline]
    fn from(value: ApiKey) -> Self {
        value.0
    }
}

impl<A: AsRef<[u8]>> PartialEq<A> for ApiKey {
    fn eq(&self, other: &A) -> bool {
        &self.0 == other.as_ref() || hex::decode(other).is_ok_and(|rhs| rhs == self.0)
    }
}
