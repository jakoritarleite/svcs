use std::cell::RefCell;

use crate::hash::create_hash;

// Node is a point in the change history
// always points to the previous node and to the next.
#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub prev: Option<String>,
    pub next: RefCell<Option<String>>,
}

impl Node {
    pub fn new(prev_id: Option<String>, next_id: Option<String>) -> Self {
        Self {
            id: create_hash(),
            prev: prev_id,
            next: RefCell::new(next_id),
        }
    }
}

