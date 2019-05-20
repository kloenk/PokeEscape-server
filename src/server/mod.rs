use colored::*;
use semver::{Version, VersionReq};
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::sync::mpsc;

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
pub fn handle_pokemon_client(
    mut stream: TcpStream,
    mut tx: mpsc::Sender<Message>,
) -> Result<TcpStream> {
    let mut reader = BufReader::new(stream.try_clone().unwrap()); //FIXME: unwrap

    // create channel
    let (txOwn, rx) = mpsc::channel();
    let mut txOwn = Some(txOwn); // encase in Option<T> so it can move out of scope in a controlled way
    let mut isIdentified = false; // set to true when it is identified to master thread

    // create empty buffer for recieved line
    let mut line = String::new();

    let mut message = Message::empty();

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
            tx.send(message.new_message(MessageBody::CLOSE)).unwrap();
            break; // exit loop
        } else if line.to_lowercase().starts_with("identify") {
            /*if !isIdentified {
                let id =  line[9..].to_string();
                tx.send(Message::IDENTIFY(Ident::new(id, txOwn))).unwrap();
                isIdentified = true;
            }*/
            match txOwn {
                Some(txO) => {
                    let id = line[9..].to_string();
                    message = Message::new_id(id.clone());
                    tx.send(message.new_message(MessageBody::IDENTIFY(Ident::new(id, txO))))
                        .unwrap();
                    //tx.send(Message::IDENTIFY(Ident::new(id, txO))).unwrap();
                    txOwn = None;
                }
                None => {
                    stream.write(b"Error")?;
                }
            }
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
#[derive(Debug)]
pub struct Ident {
    /// id of the client (generated as UUID)
    pub id: String,

    /// channel to send messages to
    pub tx: mpsc::Sender<Message>,
}

impl Ident {
    /// create a new instance of Ident
    pub fn new(id: String, tx: mpsc::Sender<Message>) -> Self {
        Ident { id, tx }
    }
}

/// struct containig ID and message for inter Thread Communication
pub struct Message {
    /// id of the client sending the message
    pub id: String,

    /// message to process
    pub message: MessageBody,
}

impl Message {
    /// create a new instance of Message shortcut
    pub fn new(id: String, message: MessageBody) -> Self {
        Message { id, message }
    }

    /// create new message with id prefilled
    pub fn new_message(&self, message: MessageBody) -> Self {
        Message {
            message,
            id: self.id.clone(),
        }
    }

    /// create new instance with ClOSE Message to save id for use in new_message
    pub fn new_id(id: String) -> Self {
        Message {
            id,
            message: MessageBody::CLOSE,
        }
    }

    /// empty creates an empty struct with Close as message for variable initialization
    pub fn empty() -> Self {
        Message {
            id: String::from("00001"),
            message: MessageBody::CLOSE,
        }
    }
}

/// enum for different jobs for a client thread
#[derive(Debug)]
pub enum MessageBody {
    /// Option to close the thread
    CLOSE,

    /// Command to identify client to server group
    /// ident struct with the content
    IDENTIFY(Ident),

    /// Command to attach to group
    AttachToGroup(String),
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
        Self { room: None, tx }
    }
}

/// handle interclient communication
pub fn server_client(rx: mpsc::Receiver<Message>, verbose: u8) {
    std::thread::spawn(move || {
        // hashmap containing the induvidual clients
        let mut clients: HashMap<String, Client> = HashMap::new();

        // hashmap containing the group of clients
        let mut groups: HashMap<String, Vec<String>> = HashMap::new();
        for recv in rx {
            match recv.message {
                MessageBody::CLOSE => {
                    match clients.remove(&recv.id) {
                        Some(client) => {
                            if verbose >= 2 {
                                println!("debug2: removing client {}", recv.id);
                            }
                            // remove client from group
                            match client.room {
                                Some(room) => {
                                    match groups.get(&room) {
                                        Some(grpup) => {
                                            if verbose >= 3 {
                                                println!(
                                                    "debug3: remove client from group {}",
                                                    room
                                                );
                                            }
                                            //groups[room].
                                            // TODO: delete from groups
                                        }
                                        None => (),
                                    }
                                }
                                None => (),
                            }
                        }
                        None => eprint!("Already deletet?"),
                    }
                }
                MessageBody::IDENTIFY(ident) => {
                    if verbose >= 2 {
                        println!("debug2: client {} identified himself", ident.id);
                    }
                    clients.insert(ident.id, Client::new(ident.tx));
                }
                MessageBody::AttachToGroup(group) => {
                    if verbose >= 2 {
                        println!("debug2: client {} joind group {}", recv.id, group);
                    }
                    match clients.get_mut(&recv.id) {
                        Some(client) => {
                            client.room = Some(group.clone());
                        }
                        None => eprintln!("ERROR: {} not in clients database", recv.id),
                    }
                    match groups.get_mut(&group) {
                        Some(group) => {
                            group.push(recv.id.clone());
                        }
                        None => {
                            let mut vec = Vec::new();
                            vec.push(recv.id);
                            groups.insert(group, vec);
                        }
                    };
                }
                _ => eprintln!("Error, could not handel : {:?}", recv.message),
            }
        }
    });
}
