// This enum defines the core directory structure precisely
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataDir {
    Data,
    KeyDefinitions,
    KeypadDefinitions,
    PanelDefinitions,
    Resources,
    I18n,
}

impl DataDir {
    /// Returns the static string representation of the directory fragment
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Data => "data",
            Self::KeyDefinitions => "key_definitions",
            Self::KeypadDefinitions => "keypad_definitions",
            Self::PanelDefinitions => "panel_definitions",
            Self::Resources => "resources",
            Self::I18n => "i18n",
        }
    }
}

impl AsRef<Path> for DataDir {
    fn as_ref(&self) -> &Path {
        Path::new(self.as_str())
    }
}

impl From<DataDir> for PathBuf {
    fn from(dir: DataDir) -> Self {
        PathBuf::from(dir.as_str())
    }
}

impl std::fmt::Display for DataDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
