mod tree;
use std::vec::Vec;

fn main() {
    let mut tree = tree::Tree::new(0);
    let new_node = tree.new_node(1, tree.root);
    tree.new_node(3, new_node);
    tree.new_node(3, new_node);
    tree.new_node(3, new_node);
    tree.new_node(3, new_node);
    tree.new_node(3, tree.root);
    tree.new_node(3, tree.root);
    tree.new_node(3, tree.root);
    tree.new_node(3, tree.root);

    tree.print();

}