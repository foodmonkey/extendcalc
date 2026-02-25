// a list of keypads from the PanelView

use crate::data::KeypadRef;

#[derive(Debug, Default, Clone)]
pub struct KeypadList {
    pub keypad_refs: Vec<KeypadRef>,
}

impl KeypadList {
    pub fn push(&mut self, keypad_ref: KeypadRef) {
        self.keypad_refs.push(keypad_ref);
    }

    pub fn len(&self) -> usize {
        self.keypad_refs.len()
    }

    pub fn contains(&self, keypad_ref: &KeypadRef) -> bool {
        self.keypad_refs.contains(keypad_ref)
    }
}

impl<'a> IntoIterator for &'a KeypadList {
    type Item = &'a KeypadRef;
    type IntoIter = std::slice::Iter<'a, KeypadRef>;

    fn into_iter(self) -> Self::IntoIter {
        self.keypad_refs.iter()
    }
}

impl<'a> IntoIterator for &'a mut KeypadList {
    type Item = &'a mut KeypadRef;
    type IntoIter = std::slice::IterMut<'a, KeypadRef>;

    fn into_iter(self) -> Self::IntoIter {
        self.keypad_refs.iter_mut()
    }
}

impl IntoIterator for KeypadList {
    type Item = KeypadRef;
    type IntoIter = std::vec::IntoIter<KeypadRef>;

    fn into_iter(self) -> Self::IntoIter {
        self.Keypad_refs.into_iter()
    }
}
