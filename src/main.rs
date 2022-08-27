
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

use std::{collections::HashSet, hash::Hash, convert::Infallible};

use arbitrary::{Arbitrary, Unstructured};

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

pub fn topological_sort_visit<'a, T>(n: &'a DAGNode<'a, T>, l: &mut Vec<&'a DAGNode<'a, T>>, permanently_marked_nodes: &mut HashSet<&'a DAGNode<'a, T>>) where T: PartialEq, T: Eq, T: Hash {
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
pub fn topological_sort<'a, T>(mut s: Vec<&'a DAGNode<'a, T>>) -> Vec<&'a DAGNode<'a, T>> where T: PartialEq, T: Eq, T: Hash { // unmarked nodes
    // Depth-first search
    let mut l = Vec::new();
    let mut permanently_marked_nodes = HashSet::new();

    while !s.is_empty() {
        let val = s.pop().unwrap();
        topological_sort_visit(&val, &mut l, &mut permanently_marked_nodes);
    }
    l
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct DAGNode<'a, T> where T: PartialEq, T: Eq, T: Hash {
    predecessors: Vec<&'a DAGNode<'a, T>>,
    current_data: T
}

pub struct CurrentState<T> {
    state: T
}

pub struct RandomDAG<'a, T>(Vec<DAGNode<'a, T>>) where T: PartialEq, T: Eq, T: Hash, T: Arbitrary<'a>;

impl<'a, T> Arbitrary<'a> for RandomDAG<'a, T>
where
T: PartialEq, T: Eq, T: Hash, 
    T: Arbitrary<'a>,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        // Get an iterator of arbitrary `T`s.
        let iter = u.arbitrary_iter::<T>()?;

        let len = u.arbitrary_len::<T>()?;

        // And then create a collection of that length!
        let mut my_collection = RandomDAG(Vec::with_capacity(len));
        for i in 0..len {
            let element = DAGNode {
                predecessors: vec![
                    &my_collection.0[u.int_in_range(0..=i)?]
                ],
                current_data: T::arbitrary(u)?
            };
            my_collection.0.push(element);
        }

        Ok(my_collection)
    }
}

pub type DAGNodeCounter<'a> = DAGNode<'a, i64>;

//pub type DAGNodeOrderedSet = DAGNode<Vec<i64>>;

//pub type DAGNodeOrderedGraph = DAGNode<Vec<i64>>;

// low memory and high performance implementation

fn main() {
    println!("Hello, world!");

   let test1 = DAGNodeCounter {
        current_data: 0,
        predecessors: vec![],
    };
    let test2 = DAGNodeCounter {
        current_data: 5,
        predecessors: vec![&test1],
    };

    println!("{:#?}", topological_sort(vec![&test2]));

}
