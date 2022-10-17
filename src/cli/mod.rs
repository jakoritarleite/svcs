pub mod init;
pub mod add;

use clap::{arg, ArgMatches, Command};

use crate::history::history::History;

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
        .subcommand(
            Command::new("add")
                .about("Create a new node in the history")
                .arg_required_else_help(true)
                .arg(arg!(<PATH> ... "Adds a node to the history"))
        )
}

pub fn command_match(matches: &ArgMatches, history: &mut History) {
    match matches.subcommand() {
        Some(("init", _sm)) => init::init(),
        Some(("add", sub_matches)) => {
            let paths = sub_matches
                .get_many::<String>("PATH")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Paths: {:?}", paths);
            add::add(history);
        }
        _ => unreachable!(),
    };
} 
