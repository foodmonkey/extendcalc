// the collection of keypads - we implement this as a type
// rather than a Vec so we can add "get" and "insert" by id
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::data::Key;
use crate::data::KeyGridRef;
use crate::ui::AsyncCountdown;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct KeyGrids {
    keys: IndexMap<KeyGridRef, Key>,
    countdown: AsyncCountdown,
}

impl KeyGrids {
    pub fn get(&self, keygrid_ref: &KeyGridRef) -> &Key {
        self.keys.get(keygrid_ref).unwrap()
    }

    pub fn insert(&mut self, keygrid_ref: &KeyGridRef, key: &Key) {
        self.keys.insert(keygrid_ref.clone(), key.clone());
    }

    pub fn len(&self) -> usize {
        self.keypad_views.len()
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
    type Item = (&'a KeyGridRef, &'a Key);
    type IntoIter = indexmap::map::Iter<'a, KeyGridRef, Key>;

    fn into_iter(self) -> Self::IntoIter {
        self.keys.iter()
    }
}

// 2. Mutable Iteration (&mut KeyGrids)
impl<'a> IntoIterator for &'a mut KeyGrids {
    type Item = (&'a KeyGridRef, &'a mut Key);
    type IntoIter = indexmap::map::IterMut<'a, KeyGridRef, Key>;

    fn into_iter(self) -> Self::IntoIter {
        self.keys.iter_mut()
    }
}

// 3. Owned Iteration (KeyGrids)
impl IntoIterator for KeyGrids {
    type Item = (KeyGridRef, Key);
    type IntoIter = indexmap::map::IntoIter<KeyGridRef, Key>;

    fn into_iter(self) -> Self::IntoIter {
        self.keys.into_iter()
    }
}
