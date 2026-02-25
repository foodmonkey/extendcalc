// ok so this is a hashmap to hold all the key "labeks" that have been
// rendered into Typest svg - so yeh we sacrafice a little bit of mem
// to have a hashmap of svg for each button and then we lookup at render_panel_view
// time to plonk it into the button

use cosmic::widget::svg;
use std::collections::HashMap;

use crate::data::KeyIdentity;
use crate::ui::AsyncCountdown;

#[derive(Debug, Clone)]
pub struct KeySvg {
    pub svgs: HashMap<KeyIdentity, svg::Handle>,
    pub countdown: AsyncCountdown,
}

impl KeySvg {
    pub fn new() -> Self {
        KeySvg {
            svgs: HashMap::new(),
            countdown: AsyncCountdown::default(),
        }
    }

    pub fn get(&self, id: &KeyIdentity) -> &svg::Handle {
        self.svgs.get(id).unwrap()
    }

    pub fn contains(&mut self, key_identity: &KeyIdentity) -> bool {
        self.svgs.contains_key(key_identity)
    }

    pub fn insert(&mut self, key_identity: KeyIdentity, handle: svg::Handle) {
        self.svgs.insert(key_identity, handle);
    }

    pub fn len(&self) -> usize {
        self.svgs.iter().count()
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
