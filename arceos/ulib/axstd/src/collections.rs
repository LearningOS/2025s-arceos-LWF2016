#[cfg(feature = "alloc")]
pub use alloc::collections::*;

use hashbrown::hash_map as base;

use axhal::misc::random;
use core::hash::BuildHasher;
use siphasher::sip::SipHasher13;
use core::fmt;
use core::fmt::Debug;

pub struct HashMap<K, V, S = RandomState> {
    base: base::HashMap<K, V, S>,
}

impl<K, V, S> Default for HashMap<K, V, S>
where
    S: Default,
{
    fn default() -> HashMap<K, V, S> {
        HashMap::with_hasher(Default::default())
    }
}

impl<K, V> HashMap<K, V, RandomState>{
    pub fn new() -> Self {
        Default::default()
    }
}

impl<K, V, S> HashMap<K, V, S>{
    pub const fn with_hasher(hash_builder: S) -> HashMap<K, V, S> {
        HashMap { base: base::HashMap::with_hasher(hash_builder) }
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter { base: self.base.iter() }
    }
}

impl<K, V, S> HashMap<K, V, S>
where
    K: Eq + core::hash::Hash,
    S: BuildHasher,
{
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.base.insert(k, v)
    }
}

pub struct Iter<'a, K: 'a, V: 'a> {
    base: base::Iter<'a, K, V>,
}

impl<K, V> Clone for Iter<'_, K, V> {
    fn clone(&self) -> Self {
        Iter { base: self.base.clone() }
    }
}

impl<K, V> Default for Iter<'_, K, V> {
    fn default() -> Self {
        Iter { base: Default::default() }
    }
}

impl<K: Debug, V: Debug> fmt::Debug for Iter<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);


    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        self.base.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }

    fn count(self) -> usize {
        self.base.len()
    }

    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.base.fold(init, f)
    }
}

pub struct RandomState {
    k0: u64,
    k1: u64,
}

impl RandomState {
    pub fn new() -> RandomState {
        let (k0, k1) = hashmap_random_keys();
        RandomState { k0, k1 }
    }
}

fn hashmap_random_keys() -> (u64, u64) {
    let key = random();
    let k1 = (key >> 64) as u64;
    let k2 = (key & 0x0000_0000_0000_0000_FFFF_FFFF_FFFF_FFFF) as u64;
    (k1, k2)
}

impl Default for RandomState {
    fn default() -> RandomState {
        RandomState::new()
    }
}

impl BuildHasher for RandomState {
    type Hasher = SipHasher13;

    fn build_hasher(&self) -> Self::Hasher {
        SipHasher13::new_with_keys(self.k0, self.k1)
    }
}