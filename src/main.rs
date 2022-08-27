use std::{cell::RefCell, collections::BTreeSet, rc::Rc};

use by_address::ByAddress;
use crdt::{topological_sort, DAGNodeCounter};

fn main() {
    println!("Hello, world!");

    let test1 = ByAddress(Rc::new(RefCell::new(DAGNodeCounter {
        current_data: 0,
        predecessors: BTreeSet::from([]),
    })));
    let test2 = ByAddress(Rc::new(RefCell::new(DAGNodeCounter {
        current_data: 5,
        predecessors: BTreeSet::from([test1]),
    })));

    println!("{:#?}", topological_sort(vec![test2]));
}
