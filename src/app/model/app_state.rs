// state of the app

#[derive(Debug, Default)]
pub enum InitState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Debug)]
pub enum AppState {
    Init(InitState),
    Ready,
    Error(String),
}

impl Default for AppState {
    fn default() -> Self {
        AppState::Init(InitState::default())
    }
}

impl std::fmt::Display for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            AppState::Init(InitState::Loading) => "Loading...",
            AppState::Init(InitState::Loaded) => "Loaded",
            AppState::Ready => "Ready",
            AppState::Error(_) => "Error",
        };
        write!(f, "{}", name)
    }
}
