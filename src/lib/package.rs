pub mod lib;

struct Package {
    Id: String,
    Name: String,
    Active: bool,
    Builders: String[],
    Movers: String[],
    Fetchers: String[],
    Placers: String[],
    Options: Options,
    ExecPath: String,
    LogPath: String,
};

// These pipes options mean they can be executed in paralell
struct Options {
    pBuilders: bool,
    pMovers: bool,
    pFetchers: bool,
    pPlacers: bool,
    Debug: bool,
    DebugLogPath: Option<String>
};
