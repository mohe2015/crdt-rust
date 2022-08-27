#![no_main]
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;
use crdt::DAGNode;
use libfuzzer_sys::fuzz_target;
use crdt::RandomDAG;
use crdt::topological_sort;
use by_address::ByAddress;

/*
/home/moritz/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-cov show -Xdemangler=rustfilt fuzz/target/x86_64-unknown-linux-gnu/release/fuzz_target_1 \
    -instr-profile=fuzz/coverage/fuzz_target_1/coverage.profdata \
    -show-line-counts-or-regions \
    -show-instantiations --format=html --output-dir=coverage/html
*/


fuzz_target!(|data: RandomDAG<i32>| {
    let mut result = topological_sort(data.0.clone());

    /*if result.len() > 1 {
        result.swap(0, 1);
    }*/
    
    assert_eq!(data.0.len(), result.len());

    let a: BTreeSet<&ByAddress<Rc<RefCell<DAGNode<i32>>>>> = data.0.iter().collect();
    let b: BTreeSet<&ByAddress<Rc<RefCell<DAGNode<i32>>>>> = result.iter().collect();

    assert_eq!(a, b);

    for i in 0..result.len() {
        for predecessor in &result[i].borrow().predecessors {
            assert!(result[0..i].contains(predecessor));
        }
    }

    // TODO check only references to earlier nodes
});
