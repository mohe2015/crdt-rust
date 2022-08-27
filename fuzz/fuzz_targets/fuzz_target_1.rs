#![no_main]
use libfuzzer_sys::fuzz_target;
use crdt::RandomDAG;
use crdt::topological_sort;

fuzz_target!(|data: RandomDAG<i32>| {
    topological_sort(data.0);
});
