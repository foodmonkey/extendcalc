// state of the app
#[derive(Debug, Default, Clone)]
pub enum InitState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Debug, Clone)]
pub enum AppState {
    Init(InitState),
    Ready,
    Error(String),
}

impl Default for AppState {
    fn default() -> Self {
        // This automatically uses InitState::Loading because of its #[default]
        AppState::Init(InitState::default())
    }
}

impl std::fmt::Display for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            AppState::Init(InitState::Loading) => "Init Loading",
            AppState::Init(InitState::Loaded) => "Init Loaded",
            AppState::Ready => "Ready",
            AppState::Error(err) => return write!(f, "Error: {}", err),
        };
        write!(f, "{}", name)
    }
}
