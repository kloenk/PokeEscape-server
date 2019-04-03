use colored::*;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

/// handling code for the http server
pub mod http;

/// This function negotiates the protocoll to use between the client and the Server
/// it calles the function of the protocoll, uses &TcpStream and a buffer as arguments
/// 
/// # panics
/// has many unwrap functions
pub fn negotiate(mut conf: Job) -> Result<(), String>{  //TODO: add return types
    let mut reader = match conf.stream.try_clone() {
      Ok(stream) => BufReader::new(stream),
      Err(err) => return Err(err.to_string()),
    };
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    if conf.verbose {
        let addr = conf.stream.peer_addr().unwrap().to_string();
        println!("got {} from {}", line.trim().yellow(), addr.green());
    }

    if line.starts_with("POKEMON-ESCAPE_") {
        // run pokemon server
        eprintln!("fix POKEMON-ESCAPE-CLIENT");
        conf.stream
            .write(format!("POKEMON-ESCAPE-SERVER_{}\n", env!("CARGO_PKG_VERSION")).as_bytes()).unwrap();
    } else if line.contains("HTTP/1.1") {
        http::handle_client(&mut conf.stream, reader).unwrap();
    } else {
        conf.stream.write(b"Protocol mismatch.").unwrap();  //FIXME: unwrap
    }
    conf.stream.flush().unwrap();
    Ok(())
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

/// Negotiates the protocol betwen the client and the server.
/// Falls back to Protocol mismatch if the clients is not a POKEMON-ESCAPE client
/// or a request starting with 'GET'
/// 'GET' is treated as http
///
/// child needs to run flush, because we transfer TcpStream to the chiled that
/// is called
pub fn hande_client(mut stream: TcpStream) -> Result<(), ()> {
    let mut buffer = [0; 512]; // FIXME: Change to string

    stream.read(&mut buffer).unwrap(); //FIXME: unwrap

    let client_id = b"POKEMON-ESCAPE_";

    if buffer.starts_with(client_id) {
        stream
            .write(format!("POKEMON-ESCAPE-SERVER_{}\n", env!("CARGO_PKG_VERSION")).as_bytes())
            .unwrap();
        stream.flush().unwrap(); //FIXME: unwrap
        handle_pokemon_client(stream).unwrap(); //FIXME: unwrap
    } else if buffer.starts_with(b"GET") {
        // http::handle_client(stream).unwrap(); //FIXME: unwrap
    } else {
        stream.write(b"Protocol mismatch.\n").unwrap(); //FIXME: unwrap
        stream.flush().unwrap(); //FIXME: unwrap
    }

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    Ok(())
}


pub struct Job {
  pub stream: TcpStream,
  pub verbose: bool,
}