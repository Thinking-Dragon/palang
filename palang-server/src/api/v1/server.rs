pub struct ServerArgs {
    pub host: String,
    pub port: u16,
}

impl ServerArgs {
    pub fn new(host: String, port: u16) -> Self {
        ServerArgs { host, port }
    }
}

pub struct AppState;

impl AppState {
    pub fn new() -> Self {
        AppState
    }
}
