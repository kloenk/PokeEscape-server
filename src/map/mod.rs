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
        let maps = Map::from_conf(&content, &String::from("test"), verbose)?;
        let version = match content["Maps"]["version"].as_str() {
            Some(ver) => ver,
            None => return Err(Error::new(super::error::ErrorKind::FieldNotExists)),
        };
        let version = Version::from_str(version)?;
        Ok(MapPlaces {
            p_version: version,
            p_maps: vec![maps],
        })
    }
}

/// this holds a single map with all of the coresponding informations
struct Map {
    // TODO: public?
    name: String,
    map: String, // json::Value
    version: Version,
}

impl Map {
    pub fn new(
        content: String, /*json::Value*/
        name: String,
        version: Version,
    ) -> Result<Self, Error> {
        //TODO: add version checking (content == Version)

        // parse version
        Ok(Map {
            name,
            version,
            map: content,
        })
    }

    pub fn from_conf(toml: &toml::Value, name: &String, verbose: bool) -> Result<Self, Error> {
        //FIXME: test if map exists
        if toml[name].is_table() {
            return Err(Error::new_field_not_exists());
        }

        let version = match toml[name]["version"].as_str() {
            Some(ver) => ver,
            None => return Err(Error::new_field_not_exists()),
        };
        let version = Version::from_str(version)?; // parse version from toml to version

        let file = match toml[name]["path"].as_str() {
            Some(file) => file,
            None => return Err(Error::new_field_not_exists()),
        };
        let file = fs::read_to_string(file)?; // load map into ram

        let format = match toml[name]["format"].as_str() {
            Some(format) => format,
            None => "json",
        };
        if format.to_lowercase() == "json" {
            // parse from json
        } else {
            return Err(Error::new(super::error::ErrorKind::FormatNotSupported));
        }

        Self::new(
            // already answers with Result
            file,         // the map itself
            name.clone(), // name of the map
            version,      // version of map
        )
    }
}
