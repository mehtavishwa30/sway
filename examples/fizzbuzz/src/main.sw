contract;

enum FizzBuzzResult {
    Fizz: (),
}

abi FizzBuzz {
    fn fizzbuzz(input: u64) -> FizzBuzzResult;
}
