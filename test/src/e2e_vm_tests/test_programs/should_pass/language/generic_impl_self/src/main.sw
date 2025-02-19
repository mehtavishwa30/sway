script;

use std::{result::*, revert::*, u128::*, assert::assert};

struct Data<T> {
  value: T
}

impl<T> Data<T> {
  fn new(v: T) -> Self {
    Data {
      value: v
    }
  }

  fn get_value(self) -> T {
    self.value
  }
}

struct DoubleIdentity<T, F> {
  first: T,
  second: F,
  third: u64
}

impl<T, F> DoubleIdentity<T, F> {
  fn new(x: T, y: F) -> DoubleIdentity<T, F> {
    DoubleIdentity {
      first: x,
      second: y,
      third: 10u64,
    }
  }

  fn get_first(self) -> T {
    let x: T = self.first;
    x
  }

  fn get_second(self) -> F {
    let y: F = self.second;
    y
  }

  fn get_third(self) -> u64 {
    let z: u64 = self.third;
    z
  }
}

impl DoubleIdentity<u8, u8> {
  fn add(self) -> u8 {
    self.first + self.second
  }
}

fn double_identity2<T, F>(x: T, y: F) -> DoubleIdentity<T, F> {
  DoubleIdentity::<T, F>::new(x, y)
}

fn double_identity<T, F>(x: T, y: F) -> DoubleIdentity<T, F> {
  let inner: T = x;
  DoubleIdentity {
    first: inner,
    second: y,
    third: 20u64,
  }
}

fn crazy<T, F>(x: T, y: F) -> F {
  let foo = DoubleIdentity {
    first: x,
    second: y,
    third: 30u64,
  };
  foo.get_second()
}

enum MyResult<T> {
  Ok: T,
  Err: u8 // err code
}

impl<T> MyResult<T> {
  fn ok(value: T) -> Self {
    MyResult::Ok::<T>(value)
  }

  fn err(code: u8) -> Self {
    MyResult::Err::<T>(code)
  }
}

enum MyOption<T> {
  Some: T,
  None: ()
}

impl<T> MyOption<T> {
  fn some(value: T) -> Self {
    MyOption::Some::<T>(value)
  }

  fn none() -> Self {
    MyOption::None::<T>
  }

  fn to_result(self) -> MyResult<T> {
    if let MyOption::Some(value) = self {
      MyResult::<T>::ok(value)
    } else {
      MyResult::<T>::err(99u8)
    }
  }
}

impl<T, E> Result<T, E>{
    fn dummy(t: T) -> Result<T, bool> {
        Result::Ok(t)
    }
}

fn result_impl_test() {
    let res = U128::from((0, 13)).as_u64();
    assert(!Result::dummy(false).unwrap());
    assert(res.unwrap_or(5) == 13);
}

fn main() -> u32 {
  let a = double_identity(true, true);
  let b = double_identity(10u32, 43u64);
  let c = double_identity2(10u8, 1u8);
  let d = DoubleIdentity {
    first: 1u8,
    second: 2u8,
    third: 40u64,
  };
  let e = d.get_second();
  let f: DoubleIdentity<bool, bool> = double_identity(true, true);
  let g: DoubleIdentity<u32, u64> = double_identity(10u32, 43u64);
  let h = DoubleIdentity::<u64, bool>::new(3u64, false);
  let i = crazy(7u8, 10u8);
  let j = 10u8 + 11u8;
  let k = d.add();
  let l = Data::<bool>::new(false);
  let m: DoubleIdentity<Data<u8>, Data<u64>> = DoubleIdentity {
    first: Data {
      value: 1u8
    },
    second: Data {
      value: 2u8
    },
    third: 1u64
  };
  let n = DoubleIdentity::<Data<u8>, Data<u8>>::new(Data::<u8>::new(3u8), Data::<u8>::new(4u8));
  let o: DoubleIdentity<bool, bool> = double_identity(true, true);
  let p = MyOption::Some::<bool>(false);
  let q = MyOption::Some::<()>(());
  let r = MyOption::<u32>::some(5u32);
  let s = MyOption::Some(0u8);
  let t = MyOption::<u64>::none();
  let u = DoubleIdentity::<Data<u8>, Data<u8>>::new(Data::<u8>::new(3u8), Data::<u8>::new(4u8));

    result_impl_test();

  b.get_first()
}
