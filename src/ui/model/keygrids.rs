// the collection of keypads - we implement this as a type
// rather than a Vec so we can add "get" and "insert" by id
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::data::Key;
use crate::ui::AsyncCountdown;
use crate::ui::KeyGridId;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct KeyGrids {
    keys: IndexMap<KeyGridId, Key>,
    countdown: AsyncCountdown,
}

impl KeyGrids {
    pub fn get(&self, keygrid_ref: &KeyGridId) -> &Key {
        self.keys.get(keygrid_ref).unwrap()
    }

    pub fn insert(&mut self, keygrid_ref: &KeyGridId, key: &Key) {
        self.keys.insert(keygrid_ref.clone(), key.clone());
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn track_async(&mut self, count: usize) {
        self.countdown.track(count);
    }

    pub fn async_remaining(&self) -> usize {
        self.countdown.remaining()
    }

    pub fn async_finished(&self) -> bool {
        self.countdown.is_zero()
    }
}

// 1. Immutable Iteration (&KeyGrids)
impl<'a> IntoIterator for &'a KeyGrids {
    type Item = (&'a KeyGridId, &'a Key);
    type IntoIter = indexmap::map::Iter<'a, KeyGridId, Key>;

    fn into_iter(self) -> Self::IntoIter {
        self.keys.iter()
    }
}

// 2. Mutable Iteration (&mut KeyGrids)
impl<'a> IntoIterator for &'a mut KeyGrids {
    type Item = (&'a KeyGridId, &'a mut Key);
    type IntoIter = indexmap::map::IterMut<'a, KeyGridId, Key>;

    fn into_iter(self) -> Self::IntoIter {
        self.keys.iter_mut()
    }
}

// 3. Owned Iteration (KeyGrids)
impl IntoIterator for KeyGrids {
    type Item = (KeyGridId, Key);
    type IntoIter = indexmap::map::IntoIter<KeyGridId, Key>;

    fn into_iter(self) -> Self::IntoIter {
        self.keys.into_iter()
    }
}
