contract;

enum FizzBuzzResult {
    Fizz: (),
}

abi FizzBuzz {
    fn fizzbuzz(input: u64) -> FizzBuzzResult;
}

impl FizzBuzz for Contract {
    fn fizzbuzz(input: u64) {}
}
