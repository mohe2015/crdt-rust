
// we need to store all past operations to allow to verify them
// so we need a DAG (signed)
// storing the DAG, querying the DAG, merging the DAG and calculating
// the current states based on the changes should be efficient
// should be possible to efficiently serialize on memory
// probably should be possible concurrently

// kind of a Delta state CRDT

// if there are several heads we trust we need to merge these locally
// for that we need the following guarantees:

// the challenge is that merges can merge nodes multiple times

// https://en.wikipedia.org/wiki/Directed_acyclic_graph
// topological order

// commutativity is required

use std::{collections::HashSet, hash::Hash};


pub fn topological_sort_visit<T>(node: DAGNode<T>) {
}

// https://en.wikipedia.org/wiki/Topological_sorting
pub fn topological_sort<T>(mut s: Vec<DAGNode<T>>) {
    // Depth-first search
    let l = Vec::new();
    let permanently_marked_nodes = HashSet::new();

    while !s.is_empty() {
        topological_sort_visit(s.pop().unwrap())
    }
}

pub struct DAGNode<T> {
    predecessors: Vec<DAGNode<T>>,
    current_data: T
}

pub struct CurrentState<T> {
    state: T
}

pub type DAGNodeCounter = DAGNode<i64>;

pub type DAGNodeOrderedSet = DAGNode<Vec<i64>>;

// low memory and high performance implementation

fn main() {
    println!("Hello, world!");

    let test1 = DAGNodeCounter {
        current_data: 0,
        predecessors: vec![],
    };
    let test2 = DAGNodeCounter {
        current_data: 5,
        predecessors: vec![test1],
    };


}
