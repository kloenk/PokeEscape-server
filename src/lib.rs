//! # Poké Escape server
//! PokéEscape is a game created in School as a group Project.
//! This project contains the source code for the server, talking
//! with the client written in GreenFoot (java)
#![deny(missing_docs)]
use colored::*;
use std::net::TcpListener;
use std::process;

/// general tcp module for talking with the client and negotiating the
/// protocoll to use
pub mod server;

/// ThreadPool module
pub mod threads;

/// module providing error Type/conversion
pub mod error;

/// module providing map loader
pub mod map;

/// struct deriving cli parsing. It also implements the run function, serving the main function
pub struct Config {
    /// configures the port to listen on
    pub port: u16,

    /// configures the interface (ip) to listen on
    pub host: String,

    /// enables verbose mode
    pub verbose: bool,

    /// defines the number of thread in ThreadPool to use
    pub threads: usize,

    /// sets the config file (toml) to load the maps
    pub config: String,
}

impl Config {
    /// new creates a struct with default values
    pub fn new() -> Self {
        Self {
            port: 1996,
            host: "127.0.0.1".to_string(),
            verbose: false,
            threads: 8,
            config: "./config.toml".to_string(),
        }
    }
    /// run function serving as the main function of the librarie.AsMut
    ///
    /// The function takes the config from itself, and serves the server as descriped
    /// in this config.
    pub fn run(&self) {
        println!(
            "Starting {} server on port: {}",
            "PokeEscape".green(),
            self.port.to_string().yellow()
        );
        println!(
            "{}: {}",
            "version".bold().white(),
            env!("CARGO_PKG_VERSION").blue()
        );
        if self.verbose {
            println!("Running in {} mode", "Verbose".red());
        }

        // load maps
        #[allow(unused_variables)]
        let maps = match map::MapPlaces::new(&self.config, self.verbose) {
            Ok(maps) => maps,
            Err(err) => {
                eprintln!("Error loading maps: {}", err.to_string().red());
                std::process::exit(20);
            }
        };

        // create ThreadPool
        let mut thread_pool = threads::ThreadPool::new(self.threads).unwrap_or_else(|err| {
            println!("Error creating threadPool: {}", err.to_string().red());
            process::exit(-2);
        });

        if self.verbose {
            thread_pool.verbose();
            println!(
                "created {} with {} workers",
                "ThreadPool".blue(),
                thread_pool.get_threads().to_string().green()
            );
        }

        println!(
            "listening on {}:{}",
            self.host.green(),
            self.port.to_string().green()
        );
        // open socket
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).unwrap(); //FIXME: !!!

        // handle incomming streams
        for stream in listener.incoming() {
            let stream = stream.unwrap(); // FIXME: unwrap
            let conf = server::Job {
                stream,
                verbose: self.verbose,
            };

            // execute Job in ThreadPool
            thread_pool
                .execute(move || {
                    server::negotiate(conf).unwrap(); //FIXME: unwrap
                })
                .unwrap(); // FIXME: unwrap
        }
    }
}
