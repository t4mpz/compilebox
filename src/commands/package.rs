use std::collections::HashMap;
use std::error;
use std::fmt;


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
        match hash.get("Options") {
            Some(options) => {
                self.Options = options.split(",").map(|b: &str| String::from(b)).collect()
            },
            None => return Err(InvalidPackageFile.into())
        }

        self.Options = opt;

        Ok(())
    }
}

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
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<fmt::Display> {
        write!(f, "Invalid options received inside package")
    }
}

impl error::Error for InvalidOptions;

// TODO
impl Options {
    fn 
};

