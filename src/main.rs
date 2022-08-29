mod hash;
mod history;

use history::history::History;

fn main() {
    let mut history = History::default();

    //println!("{:?}", history);

    history.insert_new();

    //println!("{:?}", history);

    history.insert_new();

    //println!("{:?}", history);

    history.insert_new();

    //println!("{:?}", history);

    history.print();

    history.query(String::from("ll"));
}
