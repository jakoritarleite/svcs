mod hash;
mod history;

use history::history::History;

fn main() {
    let mut history = History::new();

    println!("{:?}", history);

    history.insert_new();

    println!("{:?}", history);

    history.insert_new();

    println!("{:?}", history);

    history.insert_new();

    println!("{:?}", history);

    history.print();
}
