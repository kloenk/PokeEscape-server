use colored::*;
use semver::Version;
use std::collections::HashMap;
use std::fs;
use std::ops;
use std::str::FromStr;
use toml::Value;

use super::error::Error;

/// struct holding all informations of the toml file
/// this is not a deserialisable struct, so every map can live at top level
pub struct MapPlaces {
    p_version: Version,
    p_maps: HashMap<String, MapInfo>,
}

impl MapPlaces {
    /// parses toml file and creates a new MapPlaces instance
    pub fn new(file: &str, verbose: bool) -> Result<Self, Error> {
        if verbose {
            println!("Loading {} from {}", "Maps".green(), file.blue());
        }

        let file = fs::read_to_string(file)?;
        let content: Value = toml::from_str(file.as_str())?;

        if content.get("Maps") == None {
            return Err(Error::new_field_not_exists("Maps".to_string()));
        }

        let version = match content["Maps"]["version"].as_str() {
            Some(ver) => ver,
            None => return Err(Error::new_field_not_exists("Maps.version".to_string())),
        };
        let version = Version::from_str(version)?;

        let mut maps = HashMap::new();

        if version < Version::new(99, 99, 99) {
            // check in reserve order for version
            maps = MapInfo::from_conf(&content, verbose)?;
        }
        Ok(MapPlaces {
            p_version: version,
            p_maps: maps,
        })
    }

    /// return vector of possible map names
    pub fn available_maps(&self) -> Vec<String> {
        let mut vec = Vec::new();

        for (k, _) in &self.p_maps {
            vec.push(k.clone());
        }

        vec // return map names
    }

    /// returns the pointer to a specific map
    fn get_map(&self, name: String) -> Result<Map, Error> {    //TODO: add Result type
        match self.p_maps.get(&name) {  // FIXME foo
            Some(data) => data.load_map(),
            None => Err(Error::new_field_not_exists(name))
        }
    }

    pub fn get(&self, name: &str) -> Result<Map, Error> {
        match self.p_maps.get(name) {   // FIXME: version foo for random feature
            Some(data) => data.load_map(),
            None => Err(Error::new_field_not_exists(name.to_string()))
        }
    }
}

/// Map holds a map ready to send to a client
pub struct Map {
    p_name: String,
    p_features: Option<Vec<String>>,
    p_map: Vec<[u8; 20]>,   // 20 collums - dynamic rows
}

impl Map {
    /// returns the name of the map
    pub fn name(&self) -> String {
        self.p_name.clone() // returns a clone
    }

    /// check if the feature exists
    pub fn feature(&self, feature: &String) -> bool {
        match &self.p_features {
            Some(t) => t.contains(feature),
            None => false,
        }
    }

    /// returns a list of features of the map
    pub fn feature_list(&self) -> String {
        match &self.p_features {
            Some(t) => {
                let mut ret = String::new();
                let mut runned = false;
                for v in t {
                    if runned {
                        ret += ", ";
                    }
                    ret += v;
                    runned = true;
                }
                ret
            }
            None => "".to_string()
        }
    }
}

/// this holds a single map with all of the coresponding informations
///
/// This function willnot preload the map, but loads it when used (random??)
struct MapInfo {
    /// name of the given map
    p_name: String,

    /// file to load the map from
    p_file: String,

    /// version of the maploader to use
    p_version: Version,

    /// format of the given map file
    p_format: MapFormat,

    //TODO: add authors

    /// set to true if the mapload should operate in verbose mode
    p_verbose: bool,
}

impl MapInfo {
    /// creates a new instance of the Map
    /// This function only returns the struct with the given data filled in, so there is no logic involved
    pub fn new(
        name: String,
        file: String,
        version: Version,
        format: MapFormat,
        verbose: bool,
    ) -> Self {
        MapInfo {
            p_name: name,
            p_file: file,
            p_version: version,
            p_format: format,
            p_verbose: verbose,
        }
    }

    pub fn from_conf(toml: &toml::Value, verbose: bool) -> Result<HashMap<String, Self>, Error> {
        if toml["Maps"].get("maps") == None {
            return Err(Error::new_field_not_exists("Maps.maps".to_string()));
        }

        let maps_names = match toml["Maps"]["maps"].as_array() {
            Some(maps) => maps,
            None => return Err(Error::new_field_not_exists("Maps.maps".to_string())),
        };

        let mut maps = HashMap::new();

        for map in maps_names {
            let map = match map.as_str() {
                Some(map) => map,
                None => {
                    eprintln!("Map name could not be converted to string");
                    continue; // parse next map
                }
            };

            if verbose {
                print!("Loading infos of map {}... ", map.green());
            }

            if toml.get(map) == None {
                if verbose {
                    println!("[{}]: {}", "failed".red(), "not found in config".red());
                } else {
                    eprintln!("Map {} not found in config", map.blue());
                }
                continue;
            }

            let map = match Self::from_conf_one(&toml[map], map.to_string(), verbose) {
                Ok(map) => map,
                Err(err) => {
                    println!("[{}]: {}", "failed".red(), err.to_string().red());
                    continue;
                }
            };

            if verbose {
                println!(
                    "[{}]: Version: {}",
                    "Ok".green(),
                    map.version().to_string().blue()
                );
            }

            maps.insert(map.name().clone(), map);
        }

        Ok(maps)
    }

    fn from_conf_one(toml: &toml::Value, name: String, verbose: bool) -> Result<Self, Error> {
        let file = match toml.get("path") {
            Some(path) => path,
            None => return Err(Error::new_field_not_exists(format!("{}.path", name))),
        };
        let file = match file.as_str() {
            Some(file) => file.to_string(),
            None => return Err(Error::new_field_not_exists(format!("{}.path", name))),
        };

        let version = match toml.get("version") {
            Some(version) => version,
            None => return Err(Error::new_field_not_exists(format!("{}.version", name))),
        };
        let version = match version.as_str() {
            Some(version) => Version::from_str(version)?,
            None => return Err(Error::new_field_not_exists(format!("{}.version", name))),
        };

        let format_str = match toml.get("format") {
            Some(format_str) => match format_str.as_str() {
                Some(format) => format,
                None => "JSON",
            },
            None => "JSON",
        };

        let format: MapFormat;

        if format_str.to_lowercase() == "json" {
            format = MapFormat::JSON;
        } else {
            return Err(Error::new(super::error::ErrorKind::FormatNotSupported));
        }

        Ok(Self::new(name, file, version, format, verbose))
    }

    /// returns the name of the Map it hold information about
    pub fn name(&self) -> &String {
        &self.p_name
    }

    /// returns the version of the Map
    pub fn version(&self) -> &Version {
        &self.p_version
    }

    /// set the map into verbose mode
    pub fn verbose(&mut self) -> &mut Self {
        self.p_verbose = true;
        self
    }

    /// returns the verbose state of the Map
    pub fn is_verbose(&self) -> bool {
        self.p_verbose
    }

    /// get_map returns a finish map object ready to be send
    pub fn get_map(&self) -> Result<Map, Error> {
        Err(Error::new_field_not_exists("code".to_string()))
    }

    /// load and return a map
    pub fn load_map(&self) -> Result<Map, Error> {
        if self.p_verbose {
            println!("Loading {} from {}", self.p_name.green(), self.p_file.blue());
        }

        // read json
        let file = fs::read_to_string(&self.p_file)?;
        let content: Value = serde_json::from_str(file.as_str()).unwrap();  //FIXME: unwrap


        // get name
        let name: String = match content.get("name") {
            Some(name) => { match name.as_str() {
                Some(name) => name.to_string(),
                None => return Err(Error::new_field_not_exists("name".to_string())),
            }},
            None => return Err(Error::new_field_not_exists("name".to_string())),
        };
        if name != self.p_name {    // check name
            eprintln!("Map name {} differs from name {}", name, self.p_name);
        }

        // get feature list
        let mut features: Option<Vec<String>> = None;
        let f = content.get("features");
        if f.is_some() {
            let f = f.unwrap();
            if f.is_array() {
                let mut fe = Vec::new();
                for v in f.as_array().unwrap() {
                    if !v.is_str() {
                        return Err(Error::new_field_not_exists("features is not string".to_string()))
                    }
                    fe.push(v.as_str().unwrap().to_string());
                }
                features = Some(fe);    // attach to fe
            } else if f.is_str() {
                features = Some(vec!(f.as_str().unwrap().to_string()));
            }
        }
        drop(f);    // remove f
        features = match features { // check if it only says none
            Some(f) => {
                let mut ret: Option<Vec<String>> = Some(f.clone());
                if f.len() == 1 {
                    if f[0] == "none" {
                        ret = None;
                    }
                }
                ret
            },
            None => None,
        };


        // load map
        let mut map: Vec<[u8; 20]> = Vec::new();
        let j_map = match content.get("map") {
            Some(j) => j,
            None => return Err(Error::new_field_not_exists("map".to_string())),
        };
        let j_map = match j_map.as_array() {
            Some(j) => j,
            None => return Err(Error::new_field_not_exists("map".to_string())),
        };
        for v in j_map {
            let v = match v.as_array() {
                Some(j) => j,
                None => return Err(Error::new_field_not_exists("map".to_string())),
            };
            if v.len() < 20 {
                eprintln!("map smaller than 20 collums");
            } else if v.len() > 20 {
                eprintln!("map to big");
                return Err(Error::new_field_not_exists("map".to_string()));
            }
            let mut i = 0;  // index for row
            let mut row: [u8; 20] = [0; 20];
            for b in v {
                let b = match b.as_integer() {
                    Some(j) => j,
                    None => return Err(Error::new_field_not_exists("map".to_string())),
                };
                row[i] = b as u8;
                i += 1;
            }
            map.push(row);
        }
        drop(j_map);    // remove j_map

        Ok(
            Map{
                p_name: name,
                p_features: features,
                p_map: map,
            }
        )
    }
}

/// map format is a enum of possible loader elements for a map file
enum MapFormat {
    /// JSON as the javascript object notation
    JSON,
}