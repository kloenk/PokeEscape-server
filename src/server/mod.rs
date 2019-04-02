use super::threads::ThreadPool;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

/// handling code for the http server
pub mod http;

pub struct Server<'a> {
    /// the used ThreadPool for the server
    pool: &'a mut ThreadPool,

    /// verbose mode of the server
    do_verbose: bool, //TODO: add channels for IPC
}

impl<'a> Server<'a> {
    /// creates a new `Server` with the given number of threads
    ///
    /// # Arguments
    /// * `pool` - `ThreadPool` to use for Jobs
    ///
    /// # Errors
    ///
    /// FIXME: Error listening
    ///
    /// # Examples
    ///
    /// ```
    /// use pokemon_escape_server::server::Server;
    /// use pokemon_escape_server::threads::ThreadPool;
    /// let mut pool = ThreadPool::new(4).unwrap();
    /// let mut server = Server::new(&mut pool).unwrap();
    /// ```
    pub fn new<'b>(pool: &'b mut ThreadPool) -> Result<Server<'b>, String> {
        Ok(Server {
            pool,
            do_verbose: false,
        })
    }

    /// set Server into verbose mode
    ///
    /// will print output like wich thread is dropped if set to true
    ///
    /// # Example
    /// ```
    /// use pokemon_escape_server::server::Server;
    /// use pokemon_escape_server::threads::ThreadPool;
    /// let mut pool = ThreadPool::new(4).unwrap();
    /// let mut server = Server::new(&mut pool).unwrap();
    /// server.verbose();
    /// assert_eq!(server.is_verbose(), true);
    /// ```
    pub fn verbose(&mut self) -> &Self {
        self.do_verbose = true;
        self.pool.verbose();
        self
    }

    /// sets server in the given verbose mode
    ///
    /// # Example
    /// ## set into verbose mode
    /// ```
    /// use pokemon_escape_server::server::Server;
    /// use pokemon_escape_server::threads::ThreadPool;
    /// let mut pool = ThreadPool::new(4).unwrap();
    /// let mut server = Server::new(&mut pool).unwrap();
    /// server.set_verbose_mode(true);
    /// assert_eq!(server.is_verbose(), true);
    /// ```
    ///
    /// ## set out of verbose mode
    /// ```
    /// use pokemon_escape_server::server::Server;
    /// use pokemon_escape_server::threads::ThreadPool;
    /// let mut pool = ThreadPool::new(4).unwrap();
    /// let mut server = Server::new(&mut pool).unwrap();
    /// server.set_verbose_mode(false);
    /// assert_eq!(server.is_verbose(), false);
    /// ```
    pub fn set_verbose_mode(&mut self, mode: bool) -> &Self {
        self.do_verbose = mode;
        self.pool.set_verbose_mode(mode);
        self
    }

    /// returns if the server is running in verbose mode
    ///
    /// # Panics
    /// Panics if verbose mode of pool differs from own verbose mode
    ///
    /// # Example
    ///
    /// ## verbose does not differ
    ///
    /// ```
    /// use pokemon_escape_server::server::Server;
    /// use pokemon_escape_server::threads::ThreadPool;
    /// let mut pool = ThreadPool::new(4).unwrap();
    /// let mut server = Server::new(&mut pool).unwrap();
    /// server.verbose();
    /// assert_eq!(server.is_verbose(), true);
    /// ```
    ///
    /// ## verbose mode differs
    /// ```should_panic
    /// use pokemon_escape_server::server::Server;
    /// use pokemon_escape_server::threads::ThreadPool;
    /// let mut pool = ThreadPool::new(4).unwrap();
    /// pool.verbose(); // sets pool into verbose
    /// let mut server = Server::new(&mut pool).unwrap();
    /// assert_eq!(server.is_verbose(), false);   // panics
    /// ```
    pub fn is_verbose(&self) -> bool {
        if self.pool.is_verbose() != self.do_verbose {
            panic!("verbose mode error");
        }
        self.do_verbose
    }
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
        stream.flush().unwrap(); //FIXME: unwrap

        if line.to_lowercase().starts_with("quit") {
            // send quit
            stream.write(b"Bye").unwrap(); //FIXME: unwrap
            break;
        }
    }
    stream.flush().unwrap(); //FIXME: unwrap
    Ok(stream)
}
