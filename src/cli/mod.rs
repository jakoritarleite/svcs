pub mod init;

use clap::{ArgMatches, Command};

pub fn commands() -> Command<'static> {
    Command::new("svcs")
        .about("Simple Version Control System")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("init")
                .about("Initialize a new svcs project")
                .arg_required_else_help(false),
        )
}

pub fn command_match(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("init", _sm)) => init::init(),
        _ => unreachable!(),
    };
} 
