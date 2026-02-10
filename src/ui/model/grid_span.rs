// a struct that holds the grid span in row, column
// this will allow us to have keys that span a row or a column
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Copy, Clone)]
pub struct GridSpan {
    #[serde(default = "GridSpan::default_span")]
    pub row_span: usize,

    #[serde(default = "GridSpan::default_span")]
    pub column_span: usize,
}

impl GridSpan {
    pub fn new(row_span: usize, column_span: usize) -> Self {
        GridSpan {
            row_span,
            column_span,
        }
    }

    fn default_span() -> usize {
        1
    }
}
