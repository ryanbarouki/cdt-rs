
use crate::tree;
use crate::tree::NodeId;
use rand::seq::SliceRandom;
use rand::thread_rng;
use queues::*;
extern crate random_choice;
use self::random_choice::random_choice;


pub struct TreeGenerator {

}

impl TreeGenerator {
    pub fn generate_tree(mut iterations: i32, g: f64) -> tree::Tree<i32> {
        let mut tree = tree::Tree::new(0);
        let samples: Vec<i32> = (0..100).collect();
        let weights: Vec<f64> = samples.iter().map(|x| g.powi(-(x+1))).collect();
        let mut choices  = random_choice().random_choice_f64(&samples, &weights, iterations.try_into().unwrap());
        choices.shuffle(&mut thread_rng());
        println!("{:?}", choices);
        let mut node_queue: Queue<NodeId> = queue![];
        let mut current = tree.root;
        let mut count = 1;
        for num_children in choices {
            if *num_children == 0 {
                continue;
            }
            for _i in 0..*num_children {
                node_queue.add(tree.new_node(count, current));
                count += 1;
            }
            current = match node_queue.remove() {
                Ok(current) => current,
                Err(_) => break
            };
        }
        tree
    }
    
}