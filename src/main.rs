use std::{rc::Rc, cell::RefCell, collections::BTreeSet};

use crdt::{DAGNodeCounter, topological_sort};


fn main() {
    println!("Hello, world!");

   let test1 = Rc::new(RefCell::new(DAGNodeCounter {
        current_data: 0,
        predecessors: BTreeSet::from([]),
    }));
    let test2 = Rc::new(RefCell::new(DAGNodeCounter {
        current_data: 5,
        predecessors: BTreeSet::from([test1]),
    }));

    println!("{:#?}", topological_sort(vec![test2]));

}
