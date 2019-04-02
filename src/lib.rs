use colored::*;
use std::net::TcpListener;
use std::process;
use structopt::StructOpt;

pub mod server;

/// threads contains the struct ThreadPool and all helper functions
pub mod threads;

/// Config is a interface designed to use with structopt on the cli, but also to run the code
///
#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct Config {
    /// The port to run on
    #[structopt(short = "p", long = "port", default_value = "1996")]
    pub port: u16,

    /// address to listen on
    #[structopt(short = "H", long = "host", default_value = "127.0.0.1")]
    pub host: String,

    /// Set application into verbose mode
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,

    /// Set number of running threads
    #[structopt(short = "t", long = "threads", default_value = "8")]
    pub threads: usize,
}

impl Config {
    /// Main function for the server
    pub fn run(&self) {
        println!(
            "Starting {} server on port: {}",
            "PokemonEscape".green(),
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

        println!(
            "listening on {}:{}",
            self.host.green(),
            self.port.to_string().green()
        );
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).unwrap(); //FIXME: !!!

        let mut thread_pool = threads::ThreadPool::new(self.threads).unwrap_or_else(|err| {
            println!("Error creating threadPool: {}", err.red());
            process::exit(-2);
        });

        if self.verbose {
            thread_pool = thread_pool.verbose();
        }

        for stream in listener.incoming() {
            let stream = stream.unwrap(); //FIXME: !!!

            thread_pool
                .execute(|| {
                    server::hande_client(stream).unwrap(); //FIXME: unwrap
                })
                .unwrap(); // FIXME: unwrap
        }
    }
}
