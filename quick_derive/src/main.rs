use std::path::PathBuf;

use clap::{CommandFactory, Parser, Subcommand};
use we_clap::{WeCommand, WeParser};

impl we_clap::WeParser for Cli {}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let cli: Cli = Cli::we_parse();

    let mut msg = String::new();
    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        msg.push_str(&format!("Value for name: {name}\n"));
    }

    if let Some(config_path) = cli.config.as_deref() {
        msg.push_str(&format!("Value for config: {}\n", config_path.display()));
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => msg.push_str("Debug mode is off\n"),
        1 => msg.push_str("Debug mode is kind of on\n"),
        2 => msg.push_str("Debug mode is on\n"),
        _ => msg.push_str("Don't be crazy\n"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                msg.push_str("Printing testing lists...\n");
            } else {
                msg.push_str("Not printing testing lists...\n");
            }
        }
        None => {}
    }

    cliw::output::print(&msg);

    // Continued program logic goes here...
    // we test the we_print_long_help() function

    let _ = <Cli as CommandFactory>::command().we_print_long_help();
}
