use colored::*;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

use super::error::Error;

#[doc(inline)]
pub use super::error::Result;

/// handling code for the http server
pub mod http;

/// This function negotiates the protocoll to use between the client and the Server
/// it calles the function of the protocoll, uses &TcpStream and a buffer as arguments
pub fn negotiate(mut conf: Job) -> Result<()> { // FIXME: return error
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
            .write(format!("POKE-ESCAPE-SERVER_{}\n", env!("CARGO_PKG_VERSION")).as_bytes())?;
    } else if line.contains("HTTP/1.1") {
        http::handle_client(&mut conf.stream, reader)?;
    } else {
        conf.stream.write(b"Protocol mismatch.")?;
    }
    conf.stream.flush()?;
    Ok(()) // return type
}

/// starts the connection to the client
pub fn handle_pokemon_client(mut stream: TcpStream) -> Result<TcpStream> {
    let mut reader = BufReader::new(stream.try_clone().unwrap()); //FIXME: unwrap
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Err(_err) => return Err(Error::new_field_not_exists("fix error handling".to_string())), //FIXME: return error?
            Ok(_) => (),                 // would return usize with number read bytes
        };

        stream.write(line.as_bytes())?;
        stream.flush()?;

        if line.to_lowercase().starts_with("quit") {
            // send quit
            stream.write(b"Bye")?;
            break; // exit loop
        }
    }
    stream.flush()?;
    Ok(stream)
}

pub struct Job {
    pub stream: TcpStream,
    pub verbose: bool,
}
