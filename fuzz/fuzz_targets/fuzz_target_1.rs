#![no_main]
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;
use crdt::DAGNode;
use libfuzzer_sys::fuzz_target;
use crdt::RandomDAG;
use crdt::topological_sort;
use by_address::ByAddress;

fuzz_target!(|data: RandomDAG<i32>| {
    let result = topological_sort(data.0.clone());
    
    assert_eq!(data.0.len(), result.len());

    let a: BTreeSet<&ByAddress<Rc<RefCell<DAGNode<i32>>>>> = data.0.iter().collect();
    let b: BTreeSet<&ByAddress<Rc<RefCell<DAGNode<i32>>>>> = result.iter().collect();

    assert_eq!(a, b);

    // TODO check only references to earlier nodes
});
