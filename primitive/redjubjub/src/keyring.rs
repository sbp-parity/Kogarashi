use super::{Pair, Public, Signature};

use lazy_static::lazy_static;
use sp_core::{Pair as PairT, Public as PublicT, H256};
use sp_runtime::AccountId32;
use std::{collections::HashMap, ops::Deref};

/// Set of test accounts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Keyring {
    Alice,
    Bob,
    Charlie,
    Dave,
    Eve,
    Ferdie,
    One,
    Two,
}

impl Keyring {
    pub fn from_public(who: &Public) -> Option<Keyring> {
        Self::iter().find(|&k| &Public::from(k) == who)
    }

    pub fn from_account_id(who: &AccountId32) -> Option<Keyring> {
        Self::iter().find(|&k| &k.to_account_id() == who)
    }

    pub fn from_raw_public(who: [u8; 32]) -> Option<Keyring> {
        Self::from_public(&Public::from_raw(who))
    }

    pub fn to_raw_public(self) -> [u8; 32] {
        *Public::from(self).as_array_ref()
    }

    pub fn from_h256_public(who: H256) -> Option<Keyring> {
        Self::from_public(&Public::from_raw(who.into()))
    }

    pub fn to_h256_public(self) -> H256 {
        Public::from(self).as_array_ref().into()
    }

    pub fn to_raw_public_vec(self) -> Vec<u8> {
        Public::from(self).to_raw_vec()
    }

    pub fn to_account_id(self) -> AccountId32 {
        self.to_raw_public().into()
    }

    pub fn sign(self, msg: &[u8]) -> Signature {
        Pair::from(self).sign(msg)
    }

    pub fn pair(self) -> Pair {
        Pair::from_string(&format!("//{}", <&'static str>::from(self)), None)
            .expect("static values are known good; qed")
    }

    /// Returns an iterator over all test accounts.
    pub fn iter() -> impl Iterator<Item = Keyring> {
        <Self as strum::IntoEnumIterator>::iter()
    }

    pub fn public(self) -> Public {
        self.pair().public()
    }

    pub fn to_seed(self) -> String {
        format!("//{}", self)
    }
}

impl From<Keyring> for &'static str {
    fn from(k: Keyring) -> Self {
        match k {
            Keyring::Alice => "Alice",
            Keyring::Bob => "Bob",
            Keyring::Charlie => "Charlie",
            Keyring::Dave => "Dave",
            Keyring::Eve => "Eve",
            Keyring::Ferdie => "Ferdie",
            Keyring::One => "One",
            Keyring::Two => "Two",
        }
    }
}

lazy_static! {
    static ref PRIVATE_KEYS: HashMap<Keyring, Pair> =
        Keyring::iter().map(|i| (i, i.pair())).collect();
    static ref PUBLIC_KEYS: HashMap<Keyring, Public> = {
        PRIVATE_KEYS
            .iter()
            .map(|(&name, pair)| (name, pair.public()))
            .collect()
    };
}

impl From<Keyring> for Public {
    fn from(k: Keyring) -> Self {
        (*PUBLIC_KEYS).get(&k).unwrap().clone()
    }
}

impl From<Keyring> for AccountId32 {
    fn from(k: Keyring) -> Self {
        k.to_account_id()
    }
}

impl From<Keyring> for Pair {
    fn from(k: Keyring) -> Self {
        k.pair()
    }
}

impl From<Keyring> for [u8; 32] {
    fn from(k: Keyring) -> Self {
        *(*PUBLIC_KEYS).get(&k).unwrap().as_array_ref()
    }
}

impl From<Keyring> for H256 {
    fn from(k: Keyring) -> Self {
        (*PUBLIC_KEYS).get(&k).unwrap().as_array_ref().into()
    }
}

impl From<Keyring> for &'static [u8; 32] {
    fn from(k: Keyring) -> Self {
        (*PUBLIC_KEYS).get(&k).unwrap().as_array_ref()
    }
}

impl AsRef<[u8; 32]> for Keyring {
    fn as_ref(&self) -> &[u8; 32] {
        (*PUBLIC_KEYS).get(self).unwrap().as_array_ref()
    }
}

impl AsRef<Public> for Keyring {
    fn as_ref(&self) -> &Public {
        (*PUBLIC_KEYS).get(self).unwrap()
    }
}

impl Deref for Keyring {
    type Target = [u8; 32];
    fn deref(&self) -> &[u8; 32] {
        (*PUBLIC_KEYS).get(self).unwrap().as_array_ref()
    }
}
