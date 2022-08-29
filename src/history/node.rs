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
