// build a key from the RON file

use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::data::DataDir;
use crate::data::DataError;
use crate::data::KeyRef;
use crate::data::helpers::load_and_parse;
use crate::data::helpers::path_builder;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeyBase {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub tooltip_text: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum KeyType {
    Operand { value: f64 },
    Operator { qalc_term: String },
    Calculator { operation: String },
}

//  bow read our Keypads structure from the RON file

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Key {
    base: KeyBase,
    key_type: KeyType,
}

impl Key {
    pub fn from_ron(key_ref: &KeyRef) -> Result<Self, DataError> {
        println!("Loading key: {}", key_ref.id);
        let key_path = path_builder(
            DataDir::KeyDefinitions.as_str(),
            &key_ref.library,
            &key_ref.id,
        );
        load_and_parse::<Self>(&key_path)
    }
}

impl Deref for Key {
    type Target = KeyBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
