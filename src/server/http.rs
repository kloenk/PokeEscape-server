use colored::*;
use std::io::prelude::*;
use std::net::TcpStream;

/// This function is called when the client protocol seems to be HTTP
///
/// # Returns
/// The function flushes the stream to ensure that all data is written,
/// and then is returning the TcpStream in a Result Box.
pub fn handle_client(mut stream: TcpStream) -> Result<TcpStream, String> {
    println!(
        "Client {} requestd {}",
        stream.peer_addr().unwrap(), //FIXME: unwrap
        "http".blue()
    );
    let html_content = r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8" lang="en"/>
        <title>Pokémon Escape server</title>
    </head>
    <body>
        <h1>Please provide html content</h1>
    </body>
</html>"#;

    let content = format!(
        r#"HTTP/1.1 418 I'M A Pokemon
Server: PokémonEscape server
Content-Type: text/html; charset=utf-8
Content-Length: {}

{}"#,
        html_content.len(),
        html_content
    );
    stream.write(content.as_bytes()).unwrap(); //FIXME: unwrap
    stream.flush().unwrap(); //FIXME: unwrap
    Ok(stream)
}