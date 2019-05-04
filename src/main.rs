use clap::{App, AppSettings, Arg, SubCommand}; // clap for argument foo
use poke_escape_server::Config; // config object (also hold cli arguments)

fn main() {
    let mut app = App::new("poke_escape_server")
        .version(env!("CARGO_PKG_VERSION")) // read version from cargo
        .author("Finn Behrens <me@kloenk.de>")
        .about("Server for the Poké Escape game written in GreenFoot")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("INPUT")
                .takes_value(true)
                .help("set config file"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("set the level of verbosty"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .help("set port"),
        )
        .arg(
            Arg::with_name("host")
                .short("H")
                .long("host")
                .help("configures the inerface (ip) to listen on")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("threads")
                .short("t")
                .long("threads")
                .help("defines the number fo thread in ThreadPool to use")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("license")
                .about("show license")
                .version(env!("CARGO_PKG_VERSION"))
                .author("Finn Behrens <me@kloenk.de>")
                .setting(AppSettings::ColorAuto)
                .setting(AppSettings::ColoredHelp),
        )
        .subcommand(
            SubCommand::with_name("completion")
                .about("create completions")
                .version("0.1.0")
                .author("Finn Behrens <me@kloenk.de>")
                .arg(
                    Arg::with_name("shell")
                        .help("set the shell to create for. Tries to identify with env variable")
                        .index(1)
                        .required(false)
                        .value_name("SHELL")
                        .possible_value("fish")
                        .possible_value("bash")
                        .possible_value("zsh")
                        .possible_value("powershell")
                        .possible_value("elvish"),
                )
                .arg(
                    Arg::with_name("out")
                        .help("sets output file")
                        .value_name("FILE")
                        .short("o")
                        .long("output"),
                )
                .setting(clap::AppSettings::ColorAuto)
                .setting(clap::AppSettings::ColoredHelp),
        )
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::ColoredHelp);

    let matches = app.clone().get_matches(); // parse args

    // run subcommands
    if let Some(matches) = matches.subcommand_matches("completion") {
        completion(&matches, &mut app);
        std::process::exit(0);
    }

    drop(app); // destroy app

    if let Some(_) = matches.subcommand_matches("license") {
        show_license();
        std::process::exit(0);
    }

    let mut config = Config::new();

    if let Some(conf) = matches.value_of("config") {
        config.config = conf.to_string();
    }

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("verbose") {
        // FIXME: more vebose options
        0 => config.verbose = false,
        1 | _ => config.verbose = true,
    };

    if let Some(port) = matches.value_of("port") {
        if let Ok(port) = port.parse::<u16>() {
            config.port = port;
        }
    }

    if let Some(host) = matches.value_of("host") {
        config.host = host.to_string();
    }

    if let Some(threads) = matches.value_of("threads") {
        if let Ok(threads) = threads.parse::<usize>() {
            config.threads = threads;
        }
    }

    config.run(); // run server
}

// create completion
fn completion(args: &clap::ArgMatches, app: &mut App) {
    let shell: String = match args.value_of("shell") {
        Some(shell) => shell.to_string(),
        None => {
            let shell = match std::env::var("SHELL") {
                Ok(shell) => shell,
                Err(_) => "/bin/bash".to_string(),
            };
            let shell = std::path::Path::new(&shell);
            match shell.file_name() {
                Some(shell) => shell.to_os_string().to_string_lossy().to_string(),
                None => "bash".to_string(),
            }
        }
    };

    use clap::Shell;
    let shell_l = shell.to_lowercase();
    let shell: Shell;
    if shell_l == "fish".to_string() {
        shell = Shell::Fish;
    } else if shell_l == "zsh".to_string() {
        shell = Shell::Zsh;
    } else if shell_l == "powershell".to_string() {
        shell = Shell::PowerShell;
    } else if shell_l == "elvish".to_string() {
        shell = Shell::Elvish;
    } else {
        shell = Shell::Bash;
    }

    use std::io::Write;
    use std::io::BufWriter;
    use std::fs::File;

    /*let mut path: std::io::Write = match args.value_of("out") {
        Some(path) => match std::fs::File::create(path) {
            Ok(file) => std::io::BufWriter::new(file),
            Err(_) => std::io::stdout(),
        }, //FIXME: !!
        None => std::io::stdout(),
    }; //FIXME: do shell specific stuff*/
       //let mut path = std::io::stdout();

    let mut path = BufWriter::new(match args.value_of("out") {
        Some(x) => Box::new(File::create(&std::path::Path::new(x)).unwrap_or_else(|err| {
            eprintln!("Error opening file: {}", err);
            std::process::exit(1);
        })) as Box<Write>,
        None => Box::new(std::io::stdout()) as Box<Write>,
        
    });

    app.gen_completions_to("poke_escape_server", shell, &mut path);
}

/// prints the license to stdout
pub fn show_license() {
    println!(
        "This software is licensed under GPLv3+
PokeEscape-server  Copyright (C) 2019  Finn Behrens,
    Janina Lanz, Zoe Horsten, Malissa Schultke, Enna Freihoff
    This program comes with ABSOLUTELY NO WARRANTY;
    This is free software, and you are welcome to redistribute it
    under certain conditions;",
    )
}
 