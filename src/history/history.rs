use crate::history::node::Node;

#[derive(Debug)]
pub struct History {
    pub lenght: usize,
    pub nodes: Vec<Node>
}

impl History {
    pub fn new() -> Self {
        Self {
            lenght: 0,
            nodes: Vec::new()
        }
    }

    pub fn insert_new(&mut self) {
        let node = match self.nodes.last() {
            Some(head) => {
                let node = Node::new(Some(head.id.clone()), None);
                head.next.replace(Some(node.id.clone()));

                node
            },
            None => Node::new(None, None)
        };

        self.nodes.push(node);
        self.lenght = self.nodes.len();
    }

    pub fn query(&self, id: String) {
        todo!("history query");
    }

    pub fn print(&self) {
        println!("Nodes Lenght: {}", self.lenght);

        print!("--");
        for node in self.nodes.iter() {
            print!("{}", &node.id[..8]);
            print!("--");
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::History;

    #[test]
    fn should_create_empty_history() {
        let history = History::new();
    
        assert_eq!(history.lenght, 0);
        assert_eq!(history.nodes.is_empty(), true);
    }

    #[test]
    fn should_append_root_node_to_history() {
        let mut history = History::new();

        history.insert_new();

        assert_eq!(history.lenght, 1);
        assert_eq!(history.nodes.is_empty(), false);
    }

    #[test]
    fn should_append_node_to_history_and_link() {
        let mut history = History::new();

        history.insert_new();
    
        assert_eq!(history.lenght, 1);
        assert_eq!(history.nodes.is_empty(), false);
    
        history.insert_new();

        assert_eq!(history.lenght, 2);

        // Verify if nodes are correctly linked
        assert_eq!(history.nodes[0].prev, None);
        assert_eq!(history.nodes[0].next, RefCell::new(Some(history.nodes[1].id.clone())));

        assert_eq!(history.nodes[1].prev, Some(history.nodes[0].id.clone()));
        assert_eq!(history.nodes[1].next, RefCell::new(None));
    }
}
