use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

/// handling code for the http server
pub mod http;

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
        http::handle_client(stream).unwrap(); //FIXME: unwrap
    } else {
        stream.write(b"Protocol mismatch.\n").unwrap(); //FIXME: unwrap
        stream.flush().unwrap(); //FIXME: unwrap
    }

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
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
        stream.flush().unwrap();  //FIXME: unwrap

        if line.to_lowercase().starts_with("quit") {    // send quit
          stream.write(b"Bye").unwrap();  //FIXME: unwrap
          break;
        }
    }
    stream.flush().unwrap();  //FIXME: unwrap
    Ok(stream)
}
