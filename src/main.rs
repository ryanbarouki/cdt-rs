mod tree;
use std::vec::Vec;

fn main() {
    let mut tree = tree::Tree::new(0);
    let new_node = tree.new_node(1, tree.root);
    tree.new_node(3, new_node);
    // let mut ids: Vec<tree::NodeId> = vec![];
    // let n1 = tree.new_node(1);
    // let n2 = tree.new_node(2);
    // let n3 = tree.new_node(3);
    // let n4 = tree.new_node(4);
    // let n5 = tree.new_node(5);

    tree.print(&tree.root, String::from(""), true);

}