use structopt::StructOpt; // imported for trait use

use poke_escape_server::Config; // config object (also hold cli arguments)

fn main() {
    let config = Config::from_args(); // assemble args to config
    config.run(); // run server
}
