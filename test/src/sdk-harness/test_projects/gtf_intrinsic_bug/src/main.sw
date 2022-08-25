script;

dep test_abi;
use test_abi::Test;

fn main() {
    let test_contract = abi(Test, 0x086dcc4cea1731e995fe766e8225a7efd70b0ce6a9d653242aeffce9bca5174b);
    test_contract.test_func();
}
