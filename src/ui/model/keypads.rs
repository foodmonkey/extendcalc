// the collection of keypads - we implement this as a type
// rather than a Vec so we can add "get" and "insert" by id

use crate::ui::KeypadView;

#[derive(Debug, Clone)]
pub struct Keypads {
    keypad_views: Vec<KeypadView>,
}

impl Keypads {
    pub fn new() -> Self {
        Keypads {
            keypad_views: Vec::new(),
        }
    }

    pub fn id(&self, keypad_id: &str) -> KeypadView {
        self.keypad_views
            .iter()
            .find(|keypad| keypad.id == keypad_id)
            .unwrap()
            .clone()
    }

    pub fn insert(&mut self, keypad: KeypadView) {
        self.keypad_views.push(keypad);
    }

    pub fn len(&self) -> usize {
        self.keypad_views.len()
    }
}
impl<'a> IntoIterator for &'a Keypads {
    // What the loop yields: a reference to a Panel
    type Item = &'a KeypadView;

    // The engine: we borrow the one already built into Vec
    type IntoIter = std::slice::Iter<'a, KeypadView>;

    fn into_iter(self) -> Self::IntoIter {
        self.keypad_views.iter()
    }
}
