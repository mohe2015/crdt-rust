#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: RandomDAG| {
    topological_sort(data)
});
