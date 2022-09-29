use std::vec::Vec;

#[derive(Copy, Clone)]
pub struct NodeId {
    pub index: usize
}

pub struct Node<T> {
    pub children: Vec<NodeId>,
    value: Option<T>
}

pub struct Tree<T> {
    pub root: NodeId,
    pub nodes: Vec<Node<T>>
}

impl<T: std::fmt::Debug> Tree<T> {
    pub fn new(root_value: T) -> Self {
        Tree::<T> {
            root: NodeId { index: 0 },
            nodes: vec![Node::<T> {
                value: Some(root_value),
                children: vec![]
            }]
        }
    }

    pub fn new_node(&mut self, value: T, parent: NodeId) -> NodeId {
        let next_index = self.nodes.len();

        self.nodes[parent.index].children.push(NodeId { index: next_index });
        self.nodes.push(Node {
            children: vec![],
            value: Some(value)
        });

        NodeId { index: next_index }
    }

    pub fn print(&self) {
        self.print_recurse(&self.root, String::from(""), true);
    }

    fn print_recurse(&self, current: &NodeId, mut indent: String, last: bool) {
        let current_node = &self.nodes[current.index];
        print!("{indent}");

        if last {
            print!(" └─");
            indent += "  ";
        } else {
            print!(" ├─");
            indent += " │";
        }
        println!("{:?}", current_node.value.as_ref().unwrap());

        for (pos, child) in current_node.children.iter().enumerate() {
            self.print_recurse(child, String::from(indent.as_str()), pos == current_node.children.len() - 1);
        }
    }
}