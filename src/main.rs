
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

/*
L ← Empty list that will contain the sorted nodes
while exists nodes without a permanent mark do
    select an unmarked node n
    visit(n)

function visit(node n)
    if n has a permanent mark then
        return

    for each node m with an edge from n to m do
        visit(m)

    mark n with a permanent mark
    add n to head of L
*/

pub fn topological_sort_visit<T>(n: &DAGNode<T>, l: &mut Vec<&DAGNode<T>>, permanently_marked_nodes: &mut HashSet<DAGNode<T>>) where T: PartialEq, T: Eq, T: Hash {
    if permanently_marked_nodes.contains(&n) {
        return;
    }

    for predecessor in &n.predecessors {
        topological_sort_visit(predecessor, l, permanently_marked_nodes);
    }

    permanently_marked_nodes.insert(n);
    l.push(&n);
}

// https://en.wikipedia.org/wiki/Topological_sorting
pub fn topological_sort<T>(mut s: Vec<DAGNode<T>>) where T: PartialEq, T: Eq, T: Hash { // unmarked nodes
    // Depth-first search
    let mut l = Vec::new();
    let mut permanently_marked_nodes = HashSet::new();

    while !s.is_empty() {
        topological_sort_visit(&s.pop().unwrap(), &mut l, &mut permanently_marked_nodes);
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct DAGNode<'a, T> where T: PartialEq, T: Eq, T: Hash {
    predecessors: Vec<&'a DAGNode<'a, T>>,
    current_data: T
}

pub struct CurrentState<T> {
    state: T
}

//pub type DAGNodeCounter = DAGNode<i64>;

//pub type DAGNodeOrderedSet = DAGNode<Vec<i64>>;

//pub type DAGNodeOrderedGraph = DAGNode<Vec<i64>>;

// low memory and high performance implementation

fn main() {
    println!("Hello, world!");

   /* let test1 = DAGNodeCounter {
        current_data: 0,
        predecessors: vec![],
    };
    let test2 = DAGNodeCounter {
        current_data: 5,
        predecessors: vec![test1],
    };*/


}
