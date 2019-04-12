use colored::*;
use semver::Version;
use std::fs;
use std::str::FromStr;
use toml::Value;

use super::error::Error;

/// struct holding all informations of the toml file
/// this is not a deserialisable struct, so every map can live at top level
pub struct MapPlaces {
    p_version: Version,
    p_maps: Vec<Map>,
}

impl MapPlaces {
    /// parses toml file and creates a new MapPlaces instance
    pub fn new(file: &str, verbose: bool) -> Result<Self, Error> {
        if verbose {
            println!("Loading {} from {}", "Maps".green(), file.blue());
        }

        let file = fs::read_to_string(file)?;
        let content: Value = toml::from_str(file.as_str())?;
        let version = match content["Maps"]["version"].as_str() {
            Some(ver) => ver,
            None => return Err(Error::new(super::error::ErrorKind::FieldNotExists)),
        };
        let version = Version::from_str(version)?;

        let mut maps = Vec::new();

        if version < Version::new(99, 99, 99) {
            // check in reserve order for version
            maps = Map::from_conf(&content, verbose)?;
        }
        Ok(MapPlaces {
            p_version: version,
            p_maps: maps,
        })
    }
}

/// this holds a single map with all of the coresponding informations
///
/// This function willnot preload the map, but loads it when used (random??)
struct Map {
    /// name of the given map
    p_name: String,

    /// file to load the map from
    p_file: String,

    /// version of the maploader to use
    p_version: Version,

    /// format of the given map file
    p_format: MapFormat,

    /// set to true if the mapload should operate in verbose mode
    p_verbose: bool,
}

impl Map {
    /// creates a new instance of the Map
    /// This function only returns the struct with the given data filled in, so there is no logic involved
    pub fn new(
        name: String,
        file: String,
        version: Version,
        format: MapFormat,
        verbose: bool,
    ) -> Self {
        Map {
            p_name: name,
            p_file: file,
            p_version: version,
            p_format: format,
            p_verbose: verbose,
        }
    }

    pub fn from_conf(toml: &toml::Value, verbose: bool) -> Result<Vec<Self>, Error> {
        if !toml["Maps"]["maps"].is_array() {
            return Err(Error::new_field_not_exists());
        }

        let maps_names = match toml["Maps"]["maps"].as_array() {
            Some(maps) => maps,
            None => return Err(Error::new_field_not_exists()),
        };

        let mut maps = Vec::new();

        for map in maps_names {
            let map = match map.as_str() {
                Some(map) => map,
                None => {
                    eprintln!("Critical error reading a map!");
                    "err"
                } // FIXME: better error handling
            };
            if !toml[map].is_table() {
                eprintln!("Map {} not found in config", map);
            }

            maps.push(Self::from_conf_one(&toml[map], map.to_string(), verbose)?);
        }

        Ok(maps)
    }

    fn from_conf_one(toml: &toml::Value, name: String, verbose: bool) -> Result<Self, Error> {
        let file: String = unwrap_some(toml["path"].as_str())?.to_string();
        let version = unwrap_some(toml["version"].as_str())?;
        let version = Version::from_str(version)?;

        let format_str = match toml["format"].as_str() {
            Some(format) => format,
            None => "JSON", // defaults to json
        };

        let mut format: MapFormat;

        if format_str.to_lowercase() == "json" {
            format = MapFormat::JSON;
        } else {
            return Err(Error::new(super::error::ErrorKind::FormatNotSupported));
        }

        Ok(Self::new(name, file, version, format, verbose))
    }
}

/// unwrap_some unwraps a some value and returns a Result<T, Error>
fn unwrap_some<T>(value: Option<T>) -> Result<T, Error> {
    match value {
        Some(value) => Ok(value),
        None => Err(Error::new_field_not_exists()),
    }
}

/// map format is a enum of possible loader elements for a map file
enum MapFormat {
    /// JSON as the javascript object notation
    JSON,
}
