use colored::*;
use std::process;
use structopt::StructOpt;

pub mod server;

/// Config is a interface designed to use with structopt on the cli, but also to run the code
///
#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct Config {
    /// The port to run on
    #[structopt(short = "p", long = "port", default_value = "8")]
    pub port: u16,

    /// Set application into verbose mode
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,

    /// Set application into verbose mode
    #[structopt(short = "t", long = "threads")]
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
        if self.verbose {
            println!("Running in {} mode", "Verbose".red());
        }

        let thread_pool = server::ThreadPool::new(self.threads).unwrap_or_else(|err| {
            println!("Error creating threadPool: {}", err.red());
            process::exit(-2);
        });

        if self.verbose {
            thread_pool.verbose();
        }
    }
}
