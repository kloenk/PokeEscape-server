use std::fs;

use super::error::Error;

/// Map struct holding a map, and makring it eady to send
pub struct Map {

}


impl Map {
    /// list maps
    pub fn available_maps(info: &MapWalker) -> Result<Vec<String>, Error> {

        let paths = fs::read_dir(&info.path)?;
        let mut ret_value = Vec::new();

        for path in paths {
            let path = path?.path().display().to_string();
            if path.ends_with(".json") {
                let path = path[..(path.len()-5)].to_string();
                if info.verbose {
                    println!("found map {}", path);
                }
                ret_value.push(path);
            } else if info.verbose {
                println!("{} is not a valid map", path);
            }            
        }

        Ok(ret_value)
    }
}


/// map walker struct hold information regarding maps to search
pub struct MapWalker {
    path:   String,
    verbose: bool,

}

impl MapWalker {
    /// creates a new instance of MapWalker
    pub fn new(path: String, verbose: bool) -> Result<Self, Error> {

        Ok(MapWalker{
            path,
            verbose,
        })
    }
}
