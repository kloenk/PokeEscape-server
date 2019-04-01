use colored::*;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn hande_client(mut stream: TcpStream) {
    println!(
        "Client {} requestd {}",
        stream.peer_addr().unwrap(),
        "http".blue()
    );
    let html_content = format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8" lang="en"/>
        <title>Pokémon Escape server</title>
    </head>
    <body>
        <h1>Please provide html content</h1>
    </body>
</html>"#
    );

    let content = format!(
        r#"HTTP/1.1 200 OK
Server: PokemonEscape server
Content-Type: text/html; charset=utf-8
Size: {}

{}"#,
        html_content.len(),
        html_content
    );
    stream.write(content.as_bytes()).unwrap();
    stream.flush().unwrap();
}
