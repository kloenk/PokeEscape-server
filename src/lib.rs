//! # Poké Escape server
//! PokéEscape is a game created in School as a group Project.
//! This project contains the source code for the server, talking 
//! with the client written in GreenFoot (java)
#![deny(missing_docs)]
use colored::*;
use std::net::TcpListener;
use std::process;
use structopt::StructOpt;

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
#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct Config {
    /// configures the port to listen on
    #[structopt(short = "p", long = "port", default_value = "1996")]
    pub port: u16,

    /// configures the interface (ip) to listen on
    #[structopt(short = "H", long = "host", default_value = "127.0.0.1")]
    pub host: String,

    /// enables verbose mode
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,

    /// defines the number of thread in ThreadPool to use
    #[structopt(short = "t", long = "threads", default_value = "8")]
    pub threads: usize,

    /// sets the config file (toml) to load the maps
    #[structopt(short = "c", long = "config", default_value = "./config.toml")]
    pub config: String,

    /// show the licens under which this software is licensed
    #[structopt(long="license")]
    pub license: bool,
}

impl Config {
    /// run function serving as the main function of the librarie.AsMut
    /// 
    /// The function takes the config from itself, and serves the server as descriped
    /// in this config.
    pub fn run(&self) {
        // check if license is set to true
        if self.license {
            Self::show_license();
            std::process::exit(0);  // exit programm
        }
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

    /// prints the license to stdout
    pub fn show_license() {
        println!("This software is licensed under GPLv3+
PokeEscape-server  Copyright (C) 2019  Finn Behrens,
    Janina Lanz, Zoe Horsten, Malissa Schultke, Enna Freihoff
    This program comes with ABSOLUTELY NO WARRANTY;
    This is free software, and you are welcome to redistribute it
    under certain conditions;", )
    }
}
