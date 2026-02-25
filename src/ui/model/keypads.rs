// the collection of keypads - we implement this as a type
// rather than a Vec so we can add "get" and "insert" by id
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::data::KeypadRef;
use crate::ui::AsyncCountdown;
use crate::ui::KeypadView;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Keypads {
    keypad_views: IndexMap<KeypadRef, KeypadView>,
    countdown: AsyncCountdown,
}

impl Keypads {
    pub fn get(&self, keypad_ref: &KeypadRef) -> &KeypadView {
        self.keypad_views.get(keypad_ref).unwrap()
    }

    pub fn insert(&mut self, keypad_ref: &KeypadRef, keypad_view: &KeypadView) {
        self.keypad_views
            .insert(keypad_ref.clone(), keypad_view.clone());
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

// 1. Immutable Iteration (&Keypads) -> Gives you &KeypadView
impl<'a> IntoIterator for &'a Keypads {
    type Item = (&'a KeypadRef, &'a KeypadView);
    type IntoIter = indexmap::map::Iter<'a, KeypadRef, KeypadView>;

    fn into_iter(self) -> Self::IntoIter {
        self.keypad_views.iter()
    }
}

// 2. Owned Iteration (Keypads) -> Consumes the map, gives you KeypadView
impl IntoIterator for Keypads {
    type Item = (KeypadRef, KeypadView);
    type IntoIter = indexmap::map::IntoIter<KeypadRef, KeypadView>;

    fn into_iter(self) -> Self::IntoIter {
        self.keypad_views.into_iter()
    }
}
