mod hash;
mod history;
mod cli;

// use history::history::History;

fn main() {
    let matches = cli::commands().get_matches();

    return cli::command_match(&matches);
}
