script;

dep lib;
use lib::*;

configurable {
    Y: u64 = 42,
}

fn main() -> (u64, u64){
    (X, Y)
}
