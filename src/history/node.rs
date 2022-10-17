use std::{cell::RefCell, process};
use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};

use crate::hash::create_hash;

// Node is a point in the change history
// always points to the previous node and to the next.
#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub id: String,
    pub prev: Option<String>,
    pub next: RefCell<Option<String>>,
}

impl Node {
    pub fn new(prev_id: Option<String>, next_id: Option<String>) -> Self {
        let node = Self {
            id: create_hash(),
            prev: prev_id,
            next: RefCell::new(next_id),
        };

        let serialized_node: String = serde_json::to_string(&node).unwrap();

        match File::create(format!(".svcs/nodes/{}", node.id.clone())) {
            Ok(mut file) => file.write_all(serialized_node.as_bytes()).unwrap(),
            Err(error) => {
                eprintln!("Could not create new node due to error: {:?}", error);
                process::exit(1);
            }
        };

        node
    }
}

#[cfg(test)]
mod tests {
    use super::{Node, RefCell};

    #[test]
    fn should_create_root_node() {
        let node = Node::new(None, None);

        assert_eq!(node.prev, None);
        assert_eq!(node.next, RefCell::new(None));
    }

    #[test]
    fn should_link_node_and_root() {
        let root = Node::new(None, None);

        assert_eq!(root.prev, None);
        assert_eq!(root.next, RefCell::new(None));

        let node = Node::new(Some(root.id.clone()), None);

        assert_eq!(node.prev, Some(root.id.clone()));
        assert_eq!(node.next, RefCell::new(None));

        root.next.replace(Some(node.id.clone()));
    
        assert_eq!(root.next, RefCell::new(Some(node.id.clone())));
    }
}
