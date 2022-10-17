pub mod node;

use std::fs::File;

use crate::history::node::Node;


#[derive(Debug)]
pub struct HistoryConfig {
    pub length: usize,
    pub root_node: String,
    pub nodes: Vec<Node>,
}

impl HistoryConfig {
    pub fn create() {
        match File::create(".svcs/history") {
            Ok(..) => (),
            Err(error) => {
                eprintln!("Could not create svcs history file due to error: {:?}", error);
            }
        };
    }

    pub fn load_from_file() {
        let mut file = File::open(".svcs/history");
    }
}
