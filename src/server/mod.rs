use std::io::prelude::*;
use std::net::TcpStream;

pub mod http;

/// Negotiates the protocol betwen the client and the server.
/// Falls back to http if if the clients is not a POKEMON-ESCAPE client
pub fn hande_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let client_id = b"POKEMON-ESCAPE_";

    if buffer.starts_with(client_id) {
        stream
            .write(format!("POKEMON-ESCAPE-SERVER_{}\n", env!("CARGO_PKG_VERSION")).as_bytes())
            .unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(b"GET") {
        http::hande_client(stream);
    } else {
        stream.write(b"Protocol mismatch.\n").unwrap();
        stream.flush().unwrap();
    }

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
