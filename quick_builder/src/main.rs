use std::path::PathBuf;

use clap::{ArgAction, Command, arg, command, value_parser};
use we_clap::WeCommand;

fn main() {
    let mut cli = command!() // requires `cargo` feature
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        );

    let matches = &cli.we_get_matches_mut(); // use WeCommand function instead of get_matches

    let mut msg = String::new();
    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = matches.get_one::<String>("name") {
        msg.push_str(&format!("Value for name: {name}\n"));
    }

    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        msg.push_str(&format!("Value for config: {}\n", config_path.display()));
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches
        .get_one::<u8>("debug")
        .expect("Counts are defaulted")
    {
        0 => msg.push_str("Debug mode is off\n"),
        1 => msg.push_str("Debug mode is kind of on\n"),
        2 => msg.push_str("Debug mode is on\n"),
        _ => msg.push_str("Don't be crazy\n"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    if let Some(matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        if matches.get_flag("list") {
            // "$ myapp test -l" was run
            msg.push_str("Printing testing lists...\n");
        } else {
            msg.push_str("Not printing testing lists...\n");
        }
    }

    cliw::output::print(&msg);

    // Continued program logic goes here...
    // we test the we_print_long_help() function
    let _ = cli.we_print_long_help();
}
