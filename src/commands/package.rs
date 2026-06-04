pub struct Package {
    Id: String,
    Name: String,
    Active: bool,
    Builders: Vec<String>,
    Movers: Vec<String>,
    Fetchers: Vec<String>,
    Placers: Vec<String>,
    Options: Options,
    ExecPath: String,
    LogPath: String
}

struct Options {
    pBuilders: bool,
    pMovers: bool,
    pFetchers: bool,
    pPlacers: bool,
    Debug: bool,
    DebugLogPath: Option<String>
}
