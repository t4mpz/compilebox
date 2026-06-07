use std::collections::HashMap;
use std::error;
use std::fmt;
use std::hash::Hash;

#[derive(Debug, Clone, Hash)]
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
    LogPath: String,
    Hash: Option<String>
}

#[derive(Debug, Clone)]
struct InvalidPackageFile;

impl fmt::Display for InvalidPackageFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Package File received")
    }
}

impl error::Error for InvalidPackageFile {}

impl Package {
    fn from_hash(&mut self, hash: HashMap<String, String>, opt: Options) -> Result<(), Box<dyn error::Error>> {
        match hash.get("Id") {
            Some(id) => self.Id = id.clone(),
            None => return Err(InvalidPackageFile.into())
        }
        match hash.get("Name") {
            Some(name) => self.Name = name.clone(),
            None => return Err(InvalidPackageFile.into())
        }
        match hash.get("Active") {
            Some(active) => self.Active = active == "true",
            None => return Err(InvalidPackageFile.into())
        }
        match hash.get("Builders") {
            Some(builders) => {
                self.Builders = builders.split(",").map(|b: &str| String::from(b)).collect()
            },
            None => return Err(InvalidPackageFile.into())
        }
        match hash.get("Movers") {
            Some(movers) => {
                self.Movers = movers.split(",").map(|b: &str| String::from(b)).collect()
            },
            None => return Err(InvalidPackageFile.into())
        }
        match hash.get("Fetchers") {
            Some(fetchers) => {
                self.Fetchers = fetchers.split(",").map(|b: &str| String::from(b)).collect()
            },
            None => return Err(InvalidPackageFile.into())
        }
        match hash.get("Placers") {
            Some(placers) => {
                self.Placers = placers.split(",").map(|b: &str| String::from(b)).collect()
            },
            None => return Err(InvalidPackageFile.into())
        }
        match hash.get("ExecPath") {
            Some(execpath) => self.ExecPath = execpath.clone(),
            None => return Err(InvalidPackageFile.into())
        }
        match hash.get("LogPath") {
            Some(logpath) => self.LogPath = logpath.clone(),
            None => return Err(InvalidPackageFile.into())
        }
        self.Hash = hash.get("Hash").cloned();

        self.Options = opt;

        Ok(())
    }
    
}




#[derive(Debug, Clone, Hash)]
struct Options {
    pBuilders: bool,
    pMovers: bool,
    pFetchers: bool,
    pPlacers: bool,
    Debug: bool,
    DebugLogPath: Option<String>
}


#[derive(Debug, Clone)]
struct InvalidOptions;

impl fmt::Display for InvalidOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Package File received")
    }
}


impl error::Error for InvalidOptions {}

impl Options {
    fn from_struct(&mut self, opt: HashMap<String, String>) -> Result<(), Box<dyn error::Error>> {
        match opt.get("pBuilders") {
            Some(p_builders) => self.pBuilders = p_builders == "true",
            None => return Err(InvalidOptions.into()),
        }
        match opt.get("pMovers") {
            Some(p_movers) => self.pMovers = p_movers == "true",
            None => return Err(InvalidOptions.into()),
        }
        match opt.get("pFetchers") {
            Some(p_fetchers) => self.pFetchers = p_fetchers == "true",
            None => return Err(InvalidOptions.into()),
        }
        match opt.get("pPlacers") {
            Some(p_placers) => self.pPlacers = p_placers == "true",
            None => return Err(InvalidOptions.into()),
        }
        match opt.get("Debug") {
            Some(debug) => self.Debug = debug == "true",
            None => return Err(InvalidOptions.into()),
        }
        self.DebugLogPath = opt.get("DebugLogPath").cloned();

        Ok(())
    }
}

