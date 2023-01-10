# Unit Testing

Forc provides built-in support for building and executing tests for a package.

Tests are written as free functions with the `#[test]` attribute. For example:

```sway
#[test]
fn test_meaning_of_life() {
    assert(6 * 7 == 42);
}
```

Each test function is ran as if it were the entry point for a
[script](../sway-program-types/scripts.md). Tests "pass" if they return
successfully, and "fail" if they revert or vice versa while [testing failure](#testing-failure).

## Building and Running Tests

We can build and execute all tests within a package with the following:

```console
forc test
```

The output should look similar to this:

```console
  Compiled library "core".
  Compiled library "std".
  Compiled library "lib_single_test".
  Bytecode size is 92 bytes.
   Running 1 tests
      test test_meaning_of_life ... ok (170.652µs)
   Result: OK. 1 passed. 0 failed. Finished in 1.564996ms.
```

Visit the [`forc test`](../forc/commands/forc_test.md) command reference to find
the options available for `forc test`.

## Testing Failure

Forc supports testing failing cases for test functions declared with `#[test(should_revert)]`. For example:

```sway
#[test(should_revert)]
fn test_meaning_of_life() {
    assert(6 * 6 == 42);
}
```

Tests with `#[test(should_revert)]` considered to be passing if they are reverting.

## Calling Contracts

Unit tests can call contract functions:

```sway
contract;

abi MyContract {
    fn test_function() -> bool;
}

impl MyContract for Contract {
    fn test_function() -> bool {
        true
    }
}
```

To test the `test_function()`, a unit test like the following can be written:

```sway
#[test]
fn test_success() {
    let caller = abi(MyContract, CONTRACT_ID);
    let result = caller.test_function {}();
    assert(result == true)
}
```

It is also possible to test failure with contract calls as well:

```sway
#[test(should_revert)]
fn test_fail() {
    let caller = abi(MyContract, CONTRACT_ID);
    let result = caller.test_function {}();
    assert(result == false)
}
```

> **Note:** `CONTRACT_ID` is the id of the current contract and it is injected by `forc` automatically while using `forc test`.
