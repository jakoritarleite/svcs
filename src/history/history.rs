use crate::history::node::Node;

#[derive(Debug)]
pub struct History {
    pub lenght: usize,
    pub nodes: Vec<Node>
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
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
