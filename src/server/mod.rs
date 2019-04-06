use colored::*;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use semver::Version;

use super::error::Error;

/// handling code for the http server
pub mod http;

/// This function negotiates the protocoll to use between the client and the Server
/// it calles the function of the protocoll, uses &TcpStream and a buffer as arguments
pub fn negotiate(mut conf: Job) -> Result<(), Error> {
    let mut reader = BufReader::new(conf.stream.try_clone()?);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    if conf.verbose {
        let addr = conf.stream.peer_addr()?.to_string();
        println!("got {} from {}", line.trim().yellow(), addr.green());
    }

    if line.starts_with("POKE-ESCAPE_") {
        // run PokeEscape server
        eprintln!("fix POKE-ESCAPE-CLIENT");
        conf.stream
            .write(format!("POKE-ESCAPE-SERVER_{}\n", conf.version.to_string()).as_bytes())?;
        let v = &line[12..];
        let client = Client::new(v, conf.stream.try_clone()?)?;
        if conf.verbose {
            println!("Client with version {} connected", client.version.to_string());
        }
    } else if line.contains("HTTP/1.1") {
        http::handle_client(&mut conf.stream, reader)?;
    } else {
        conf.stream.write(b"Protocol mismatch.")?;
    }
    conf.stream.flush()?;
    Ok(()) // return type
}

pub struct Job {
    pub stream: TcpStream,
    pub verbose: bool,
    pub version: Version,
}

pub struct Client {
    pub version: Version,
    pub stream: TcpStream,
}

impl Client {
    pub fn new(version: &str, stream: TcpStream) -> Result<Self, Error> {
        Ok(Client {
            version: Version::parse(version)?,
            stream: stream,
        })
    }
}