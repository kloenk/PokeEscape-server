use colored::*;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::sync::{mpsc};
use semver::{Version, VersionReq};
use std::collections::HashMap;

use super::error::Error;

#[doc(inline)]
pub use super::error::Result;

/// handling code for the http server
pub mod http;

/// This function negotiates the protocoll to use between the client and the Server
/// it calles the function of the protocoll, uses &TcpStream and a buffer as arguments
pub fn negotiate(mut conf: Job) -> Result<()> {
    // FIXME: return error
    let mut reader = BufReader::new(conf.stream.try_clone()?);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    if conf.verbose {
        let addr = conf.stream.peer_addr()?.to_string();
        println!("got {} from {}", line.trim().yellow(), addr.green());
    }

    if line.starts_with("POKE-ESCAPE_") {
        conf.stream
            .write(format!("POKE-ESCAPE-SERVER_{}\n", env!("CARGO_PKG_VERSION")).as_bytes())?;
        // parse version of client
        let clientv = Version::parse(&line[12..])?;

        if conf.verbose {
            println!("Client with version {} connected", clientv);
        }

        // compare version of client
        let requirment = VersionReq::parse("<= 0.1.0").unwrap();

        if requirment.matches(&clientv) {
            handle_pokemon_client(conf.stream.try_clone()?, conf.sender);
        } else {
            conf.stream.write(b"Protocol mismatch.\n")?;
        }

    } else if line.contains("HTTP/1.1") {
        http::handle_client(&mut conf.stream, reader)?;
    } else {
        conf.stream.write(b"Protocol mismatch.\n")?;
    }
    conf.stream.flush()?;
    Ok(()) // return type
}

/// starts the connection to the client
pub fn handle_pokemon_client(mut stream: TcpStream, mut tx: mpsc::Sender<Message>) -> Result<TcpStream> {
    let mut reader = BufReader::new(stream.try_clone().unwrap()); //FIXME: unwrap

    // create channel
    let (txOwn, rx) = mpsc::channel();

    // create empty buffer for recieved line
    let mut line = String::new();

    loop {
        match reader.read_line(&mut line) {
            Err(_err) => {
                return Err(Error::new_field_not_exists(
                    "fix error handling".to_string(),
                ))
            } //FIXME: return error?
            Ok(_) => (), // would return usize with number read bytes
        };

        stream.write(line.as_bytes())?;
        stream.flush()?;

        //line.trim()

        if line.to_lowercase().starts_with("quit") {
            // send quit
            stream.write(b"Bye")?;
            tx.send(Message::CLOSE).unwrap();
            break; // exit loop
        } else if line.to_lowercase().starts_with("identify") {
            let id =  line[9..].to_string();
            tx.send(Message::IDENTIFY(Ident::new(id, txOwn))).unwrap();
        }
    }
    stream.flush()?;
    Ok(stream)
}

/// Job as parameter for negotiate to give the TCPStream and the verbose state
pub struct Job {
    /// TcpStream of the client
    pub stream: TcpStream,

    /// verbose state
    pub verbose: bool,

    /// channel to communicate with scheduler
    pub sender: mpsc::Sender<Message>,

}

/// struct for the identification of the client
pub struct Ident {
    /// id of the client (generated as UUID)
    pub id: String,
    
    // channel to send messages to
    pub tx: mpsc::Sender<Message>,
}

impl Ident {
    /// create a new instance of Ident
    pub fn new(id: String, tx: mpsc::Sender<Message>) -> Self {
        Ident {
            id,
            tx,
        }
    }
}



/// enum for different jobs for a client thread
pub enum Message {
    /// Option to close the thread
    CLOSE,

    /// Command to identify client to server group
    /// ident struct with the content
    IDENTIFY(Ident),
}

/// struct used in hashmap of the coordinator
struct Client {
    /// room of the client
    pub room: Option<String>,

    /// sender for the client
    pub tx: mpsc::Sender<Message>,
}

impl Client {
    /// create a new instance of the client
    pub fn new(tx: mpsc::Sender<Message>) -> Self {
        Self {
            room: None,
            tx,
        }
    }
}


/// handle interclient communication
pub fn server_client(rx: mpsc::Receiver<Message>) {
    std::thread::spawn(move || {
        // hashmap containing the induvidual clients
        let mut clients = HashMap::new();

        // hashmap containing the group of clients
        //let mut groups = HashMap::new();
        for recv in rx {
            match recv {
                Message::CLOSE => println!("Send close"),
                Message::IDENTIFY(ident) => {
                    println!("send identify from {}", ident.id);
                    clients.insert(ident.id, Client::new(ident.tx));
                },
                _ => println!("foo"),
            };
        }
    });
}