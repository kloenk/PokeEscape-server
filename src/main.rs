use structopt::StructOpt;

use pokemon_escape_server::Config;

fn main() {
    let config = Config::from_args();
    config.run();
}
