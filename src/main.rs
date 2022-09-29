mod tree;
mod tree_generator;

fn main() {
    let mut tree = tree_generator::TreeGenerator::generate_tree(100000, 2.0);
    tree.print();
}