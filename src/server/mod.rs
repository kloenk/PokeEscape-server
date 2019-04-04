use colored::*;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

/// handling code for the http server
pub mod http;

/// This function negotiates the protocoll to use between the client and the Server
/// it calles the function of the protocoll, uses &TcpStream and a buffer as arguments
pub fn negotiate(mut conf: Job) -> Result<(), String>{  //TODO: add return types
    let mut reader = match conf.stream.try_clone() {
      Ok(stream) => BufReader::new(stream),
      Err(err) => return Err(err.to_string()),  // return type
    };
    let mut line = String::new();
    match reader.read_line(&mut line) {
        Ok(_) => (),    // would return read bytes
        Err(err) => return Err(err.to_string()),        // return type
    };
    if conf.verbose {
        let addr = match conf.stream.peer_addr() {
            Ok(addr) => addr.to_string(),
            Err(err) => return Err(err.to_string()),    // return type
        };
        println!("got {} from {}", line.trim().yellow(), addr.green());
    }

    if line.starts_with("POKEMON-ESCAPE_") {
        // run pokemon server
        eprintln!("fix POKEMON-ESCAPE-CLIENT");
        match conf.stream
            .write(format!("POKEMON-ESCAPE-SERVER_{}\n", env!("CARGO_PKG_VERSION")).as_bytes()) {
                Ok(_) => (),    // would return bytes written
                Err(err) => return Err(err.to_string()),    // return type
            };
    } else if line.contains("HTTP/1.1") {
        match http::handle_client(&mut conf.stream, reader) {
            Ok(stream) => stream,
            Err(err) => return Err(err.to_string()),    // return type
        };
    } else {
        match conf.stream.write(b"Protocol mismatch.") {
            Ok(_) => (),    // would return bytes written
            Err(err) => return Err(err.to_string()),    //return type
        };  //FIXME: unwrap
    }
    match conf.stream.flush() {
        Ok(_) => (),
        Err(err) => return Err(err.to_string()),    // return type
    };
    Ok(())  // return type
}

/// starts the connection to the client
pub fn handle_pokemon_client(mut stream: TcpStream) -> Result<TcpStream, ()> {
    let mut reader = BufReader::new(stream.try_clone().unwrap()); //FIXME: unwrap
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Err(_err) => return Err(()), //FIXME: return error?
            Ok(_) => (),                 // would return usize with number read bytes
        };

        stream.write(line.as_bytes()).unwrap(); //FIXME: unwrap
        stream.flush().unwrap(); //FIXME: unwrap

        if line.to_lowercase().starts_with("quit") {
            // send quit
            stream.write(b"Bye").unwrap(); //FIXME: unwrap
            break; // exit loop
        }
    }
    stream.flush().unwrap(); //FIXME: unwrap
    Ok(stream)
}


pub struct Job {
  pub stream: TcpStream,
  pub verbose: bool,
}