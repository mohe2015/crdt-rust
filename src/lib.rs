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

use std::{cell::RefCell, collections::BTreeSet, rc::Rc};

use arbitrary::{Arbitrary, Unstructured};
use by_address::ByAddress;


// https://doc.rust-lang.org/std/pin/index.html
// https://arunanshub.hashnode.dev/self-referential-structs-in-rust

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

pub fn topological_sort_visit<T>(
    n: ByAddress<Rc<RefCell<DAGNode<T>>>>,
    l: &mut Vec<ByAddress<Rc<RefCell<DAGNode<T>>>>>,
    permanently_marked_nodes: &mut BTreeSet<ByAddress<Rc<RefCell<DAGNode<T>>>>>,
) where
    T: PartialEq,
    T: Eq,
    T: Ord,
{
    // TODO FIXME keep the cycle detection

    if permanently_marked_nodes.contains(&n) {
        return;
    }

    for predecessor in &n.borrow().predecessors {
        topological_sort_visit(predecessor.to_owned(), l, permanently_marked_nodes);
    }

    permanently_marked_nodes.insert(n.to_owned());
    l.push(n);
}

// https://en.wikipedia.org/wiki/Topological_sorting
pub fn topological_sort<T>(mut s: Vec<ByAddress<Rc<RefCell<DAGNode<T>>>>>) -> Vec<ByAddress<Rc<RefCell<DAGNode<T>>>>>
where
    T: PartialEq,
    T: Eq,
    T: Ord,
{
    // unmarked nodes
    // Depth-first search
    let mut l = Vec::new();
    let mut permanently_marked_nodes = BTreeSet::new();

    while !s.is_empty() {
        let val = s.pop().unwrap();
        topological_sort_visit(val, &mut l, &mut permanently_marked_nodes);
    }
    l
}

// technically this is a multi dag as there can be multiple roots and multiple heads (there may be usecases where multiple people create concurrently)
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct DAGNode<T>
where
    T: PartialEq,
    T: Eq,
    T: Ord,
{
    pub predecessors: BTreeSet<ByAddress<Rc<RefCell<DAGNode<T>>>>>,
    pub current_data: T,
}

#[derive(Debug)]
pub struct RandomDAG<T>(pub Vec<ByAddress<Rc<RefCell<DAGNode<T>>>>>)
where
    T: PartialEq,
    T: Eq,
    T: Ord,
    for<'a> T: Arbitrary<'a>,
    T: core::fmt::Debug;

impl<'a, T> Arbitrary<'a> for RandomDAG<T>
where
    T: PartialEq,
    T: Eq,
    T: Ord,
    T: for<'b> arbitrary::Arbitrary<'b>,
    T: core::fmt::Debug,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let len = u.arbitrary_len::<T>()?;

        // somebody needs to own this stuff so this is really hard

        // And then create a collection of that length!
        let mut my_collection: RandomDAG<T> = RandomDAG(Vec::with_capacity(len));
        for _ in 0..len {
            let element = DAGNode {
                predecessors: BTreeSet::new(),
                current_data: T::arbitrary(u)?,
            };
            my_collection.0.push(ByAddress(Rc::new(RefCell::new(element))));
        }
        if len > 1 {
            for _ in 0..u.int_in_range(0..=len * 10)? {
                let b = &my_collection.0;
                let index_1 = u.int_in_range(0..=len - 2)?;
                let c = b[index_1].to_owned();
                let d = b[u.int_in_range(index_1 + 1..=len - 1)?].to_owned();
                d.borrow_mut().predecessors.insert(c);
            }
        }

        Ok(my_collection)
    }
}

pub type DAGNodeCounter<'a> = DAGNode<i64>;

//pub type DAGNodeOrderedSet = DAGNode<Vec<i64>>;

//pub type DAGNodeOrderedGraph = DAGNode<Vec<i64>>;

// low memory and high performance implementation
