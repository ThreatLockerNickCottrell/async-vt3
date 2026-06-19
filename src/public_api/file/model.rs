#![allow(dead_code)]
use std::{borrow::Borrow, fmt::{Display, LowerHex}, str::FromStr};

use hex::{FromHex, FromHexError, ToHex};

use super::Attributes;
use crate::public_api::model::{Collection, Object};

pub type VtFile = Object<Attributes>;
pub type VtFiles = Collection<Attributes>;

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub enum Hash {
    Md5(Md5),
    Sha1(Sha1),
    Sha256(Sha256),
}

impl From<Md5> for Hash {
    fn from(value: Md5) -> Self {
        Self::Md5(value)
    }
}
impl From<Sha1> for Hash {
    fn from(value: Sha1) -> Self {
        Self::Sha1(value)
    }
}
impl From<Sha256> for Hash {
    fn from(value: Sha256) -> Self {
        Self::Sha256(value)
    }
}
impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        match self {
            Hash::Md5(md5) => md5.as_ref(),
            Hash::Sha1(sha1) => sha1.as_ref(),
            Hash::Sha256(sha256) => sha256.as_ref(),
        }
    }
}
impl<A> AsRef<A> for Hash
where
    Md5: AsRef<A>,
    Sha1: AsRef<A>,
    Sha256: AsRef<A>,
{
    fn as_ref(&self) -> &A {
        match self {
            Hash::Md5(md5) => md5.as_ref(),
            Hash::Sha1(sha1) => sha1.as_ref(),
            Hash::Sha256(sha256) => sha256.as_ref(),
        }
    }
}
impl Borrow<[u8]> for Hash {
    fn borrow(&self) -> &[u8] {
        match self {
            Hash::Md5(md5) => md5.borrow(),
            Hash::Sha1(sha1) => sha1.borrow(),
            Hash::Sha256(sha256) => sha256.borrow(),
        }
    }
}
impl<A> Borrow<A> for Hash
where
    Md5: Borrow<A>,
    Sha1: Borrow<A>,
    Sha256: Borrow<A>,
{
    fn borrow(&self) -> &A {
        match self {
            Hash::Md5(md5) => md5.borrow(),
            Hash::Sha1(sha1) => sha1.borrow(),
            Hash::Sha256(sha256) => sha256.borrow(),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Md5([u8; 16]);

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Sha1([u8; 20]);

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Sha256([u8; 32]);

impl Md5 {
    #[inline]
    pub const fn new(hash: [u8; 16]) -> Self {
        Self(hash)
    }
}

impl Sha1 {
    #[inline]
    pub const fn new(hash: [u8; 20]) -> Self {
        Self(hash)
    }
}

impl Sha256 {
    #[inline]
    pub const fn new(hash: [u8; 32]) -> Self {
        Self(hash)
    }
}

impl AsRef<[u8]> for Md5 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Borrow<[u8]> for Md5 {
    #[inline]
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for Sha1 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Borrow<[u8]> for Sha1 {
    #[inline]
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for Sha256 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Borrow<[u8]> for Sha256 {
    #[inline]
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}

impl FromHex for Hash {
    type Error = FromHexError;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        if let Ok(md5) = Md5::from_hex(&hex) {
            Ok(Self::Md5(md5))
        } else if let Ok(sha1) = Sha1::from_hex(&hex) {
            Ok(Self::Sha1(sha1))
        } else if let Ok(sha256) = Sha256::from_hex(hex) {
            Ok(Self::Sha256(sha256))
        } else {
            Err(FromHexError::InvalidStringLength)
        }
    }
}

impl FromHex for Md5 {
    type Error = FromHexError;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        hex::decode(hex)?
            .as_array::<16>()
            .copied()
            .map(Self)
            .ok_or(FromHexError::InvalidStringLength)
    }
}

impl FromHex for Sha1 {
    type Error = FromHexError;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        hex::decode(hex)?
            .as_array::<20>()
            .copied()
            .map(Self)
            .ok_or(FromHexError::InvalidStringLength)
    }
}

impl FromHex for Sha256 {
    type Error = FromHexError;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        hex::decode(hex)?
            .as_array::<32>()
            .copied()
            .map(Self)
            .ok_or(FromHexError::InvalidStringLength)
    }
}

impl FromStr for Hash {
    type Err = <Self as FromHex>::Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s)
    }
}

impl FromStr for Md5 {
    type Err = <Self as FromHex>::Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s)
    }
}

impl FromStr for Sha1 {
    type Err = <Self as FromHex>::Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s)
    }
}

impl FromStr for Sha256 {
    type Err = <Self as FromHex>::Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s)
    }
}
impl LowerHex for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.encode_hex::<String>().as_str())
    }
}
impl LowerHex for Md5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.encode_hex::<String>().as_str())
    }
}
impl LowerHex for Sha1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.encode_hex::<String>().as_str())
    }
}
impl LowerHex for Sha256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.encode_hex::<String>().as_str())
    }
}
impl Display for Hash {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        LowerHex::fmt(&self, f)
    }
}

impl From<Hash> for String {
    #[inline]
    fn from(value: Hash) -> Self {
        value.to_string()
    }
}
impl Display for Md5 {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        LowerHex::fmt(&self, f)
    }
}

impl From<Md5> for String {
    #[inline]
    fn from(value: Md5) -> Self {
        value.to_string()
    }
}
impl Display for Sha1 {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        LowerHex::fmt(&self, f)
    }
}

impl From<Sha1> for String {
    #[inline]
    fn from(value: Sha1) -> Self {
        value.to_string()
    }
}
impl Display for Sha256 {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        LowerHex::fmt(&self, f)
    }
}

impl From<Sha256> for String {
    #[inline]
    fn from(value: Sha256) -> Self {
        value.to_string()
    }
}

impl<A: AsRef<[u8]>> PartialEq<A> for Hash {
    fn eq(&self, other: &A) -> bool {
        <Self as AsRef<[u8]>>::as_ref(&self) == other.as_ref()
            || hex::decode(other).is_ok_and(|rhs| rhs == <Self as AsRef<[u8]>>::as_ref(&self))
    }
}
impl<A: AsRef<[u8]>> PartialEq<A> for Md5 {
    fn eq(&self, other: &A) -> bool {
        &self.0 == other.as_ref() || hex::decode(other).is_ok_and(|rhs| rhs == self.0)
    }
}
impl<A: AsRef<[u8]>> PartialEq<A> for Sha1 {
    fn eq(&self, other: &A) -> bool {
        &self.0 == other.as_ref() || hex::decode(other).is_ok_and(|rhs| rhs == self.0)
    }
}
impl<A: AsRef<[u8]>> PartialEq<A> for Sha256 {
    fn eq(&self, other: &A) -> bool {
        &self.0 == other.as_ref() || hex::decode(other).is_ok_and(|rhs| rhs == self.0)
    }
}
