// a struct that holds the grid position in row, column
// so we can use it as a key into a HashMap and retrieve
// any Key from the Grid(HashMap) and iterate over the Grid
// in row, column order or column row order it we want to

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct GridPosition {
    pub row: usize,
    pub column: usize,
}

impl GridPosition {
    pub fn new(row: usize, column: usize) -> Self {
        GridPosition { row, column }
    }
}
