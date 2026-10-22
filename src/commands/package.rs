use std::collections::HashMap;
use std::error;
use std::fmt;
use std::hash::Hash;

#[derive(Hash, Clone, Debug)]
pub enum CommandType {
    Shell,
    GitClone,
    GitPull,
}

#[derive(Debug, Clone)]
struct InvalidCommandTypeOption;

impl fmt::Display for InvalidCommandTypeOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid command type received")
    }
}


impl error::Error for InvalidCommandTypeOption {}

pub fn string_to_command_type(cmdType: String) -> Result<CommandType, Box<dyn error::Error>> {
    match cmdType.as_str() {
       "Shell" => Ok(CommandType::Shell),
       "gitClone" => Ok(CommandType::GitClone),
       "gitPull" => Ok(CommandType::GitPull),
        _ => Err(InvalidCommandTypeOption.into())
    }
}

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
pub struct Options {
    shellModeBuilders: String,
    shellModeMovers: String,
    shellModeFetchers: String,
    shellModePlacers: String,


    CommandTypeBuild: CommandType,
    CommandTypeMove: CommandType,
    CommandTypeFetch: CommandType,
    CommandTypePlace: CommandType,

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
        match opt.get("shellModeBuilders") {
            Some(p_builders) => self.shellModeBuilders = p_builders.clone(),
            None => return Err(InvalidOptions.into()),
        }
        match opt.get("shellModeMovers") {
            Some(p_movers) => self.shellModeMovers = p_movers.clone(),
            None => return Err(InvalidOptions.into()),
        }
        match opt.get("shellModeFetchers") {
            Some(p_fetchers) => self.shellModeFetchers = p_fetchers.clone() ,
            None => return Err(InvalidOptions.into()),
        }
        match opt.get("shellModePlacers") {
            Some(p_placers) => self.shellModePlacers = p_placers.clone(),
            None => return Err(InvalidOptions.into()),
        } 
        match opt.get("CommandTypeBuild") {
            Some(t_builders) => self.CommandTypeBuild = string_to_command_type(t_builders.clone()).unwrap(),
            None => return Err(InvalidOptions.into()),
        }
        match opt.get("CommandTypeMove") {
            Some(t_movers) => self.CommandTypeMove= string_to_command_type(t_movers.clone()).unwrap(),
            None => return Err(InvalidOptions.into()),
        }
        match opt.get("CommandTypeFetch") {
            Some(t_fetchers) => self.CommandTypeFetch= string_to_command_type(t_fetchers.clone()).unwrap() ,
            None => return Err(InvalidOptions.into()),
        }
        match opt.get("CommandTypePlace") {
            Some(t_placers) => self.CommandTypePlace= string_to_command_type(t_placers.clone()).unwrap(),
            None => return Err(InvalidOptions.into()),
        }
        match opt.get("Debug") {
            Some(debug) => self.Debug = debug == "true" ,
            None => return Err(InvalidOptions.into()),
        }
        self.DebugLogPath = opt.get("DebugLogPath").cloned();

        Ok(())
    }
}


