use rand::Rng;
use std::cell::RefCell;
use sha2::{Sha256, Digest};

// Node is a point in the change history
// always points to the previous node and to the next.
#[derive(Debug)]
struct Node {
    id: String,
    prev: Option<String>,
    next: RefCell<Option<String>>,
}

impl Node {
    fn new<'a>(prev_id: Option<String>, next_id: Option<String>) -> Self {
        let mut hasher = Sha256::new();

        let mut rng = rand::thread_rng();
        let nonce: u32 = rng.gen();

        hasher.update(nonce.to_string());
        let result = hasher.finalize();

        Self {
            id: hex::encode(result.as_slice()),
            prev: prev_id,
            next: RefCell::new(next_id)
        }
    }
}

#[derive(Debug)]
struct History {
    lenght: usize,
    nodes: Vec<Node>
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

impl History {
    fn new() -> Self {
        Self {
            lenght: 0,
            nodes: Vec::new()
        }
    }

    fn insert_new(&mut self) {
        if let Some(head) = self.nodes.last() {
            let node = Node::new(Some(head.id.clone()), None);
            head.next.replace(Some(node.id.clone()));
        
            self.nodes.push(node);
        } else {
            let node = Node::new(None, None);
            self.nodes.push(node);
        }

        self.lenght = self.nodes.len();
    }
}

fn print_history(history: &History) {
    println!("Nodes: {}", history.lenght);

    print!("--");
    for node in history.nodes.iter() {
        print!("{}", &node.id[..5]);
        print!("--");
    }
    println!();
}

fn main() {
    let mut history = History::default();

    println!("{:?}", history);

    history.insert_new();

    println!("{:?}", history);

    history.insert_new();

    println!("{:?}", history);

    history.insert_new();

    println!("{:?}", history);

    print_history(&history);
}
