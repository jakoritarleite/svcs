mod hash;
mod history;
mod cli;
mod config;

use history::history::History;
use config::HistoryConfig;

// Base the cli in this project here
// https://github.com/surrealdb/surrealdb/blob/main/src/cli/sql.rs

fn main() {
    let history_config = HistoryConfig::load_from_file();

    let mut history = History::new();

    let matches = cli::commands().get_matches();

    cli::command_match(&matches, &mut history);

    history.print();
}
