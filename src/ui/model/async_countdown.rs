// this is a countdown for the async loading tasks - it goes to zero

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct AsyncCountdown {
    counter: usize,
}

impl AsyncCountdown {
    pub fn track(&mut self, total: usize) {
        if self.counter == 0 {
            self.counter = total;
        }
        self.counter -= 1;
    }

    pub fn remaining(&self) -> usize {
        self.counter
    }

    pub fn is_zero(&self) -> bool {
        self.counter == 0
    }
}
