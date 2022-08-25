contract;

dep test_abi;
use test_abi::Test;
use std::logging::log;

impl Test for Contract {
    fn test_func() {
        let index = 0;
        let type_1 = __gtf::<u8>(index, 0x001);
        let type_2 = asm(r1, r2: index) {
            gtf r1 r2 i1;
            r2: u8
        };
        log(type_1);
        log(type_2);
    }
}