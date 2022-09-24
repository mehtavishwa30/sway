contract;

use std::iter::Iterator;

struct MyVec {
    counter: u64,
    item1: u64,
    item2: u64,
}

impl Iterator for MyVec {
    fn next(self) -> Option<u64> {
        match self.counter {
            0 => Option::Some(self.item1),
            1 => Option::Some(self.item2),
            _ => Option::None,
        }
    }
}

abi MyContract {
    fn test_function() -> bool;
}

impl MyContract for Contract {
    fn test_function() -> bool {
        true
    }
}
