use colored::*;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use semver::Version;

use super::error::Error;
use super::map::*;

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
        let mut client = Client::new(v, conf.stream.try_clone()?, conf.path)?;
        if conf.verbose {
            println!("Client with version {} connected", client.version.to_string());
            client.verbose();   // set client to verbose
        }

        // start client protocoll
        if client.version < Version::new(10000, 0, 0) {     // check is less (newer version ontop)
            client_0_0_0(client)?;
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
    pub path: String,
}

pub struct Client {
    pub version: Version,
    pub stream: TcpStream,
    pub p_verbose: bool,
    pub path: String,
}

impl Client {
    pub fn new(version: &str, stream: TcpStream, path: String) -> Result<Self, Error> {
        Ok(Client {
            version: Version::parse(version)?,
            stream: stream,
            path,
            p_verbose: false,
        })
    }

    pub fn verbose(&mut self) -> &mut Self {
        self.p_verbose = true;
        self
    }
}

/// PokeEscape server-client protocoll version starting with client 0.0.0
pub fn client_0_0_0(client: Client) -> Result<(), Error> {

    Map::available_maps(&MapWalker::new(client.path, client.p_verbose)?)?;

    Ok(())
}