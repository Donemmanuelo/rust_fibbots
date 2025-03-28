use num_bigint::BigInt;
use num_bigint::ToBigInt;

pub fn fibonacci(c: u128) -> BigInt {

  match  c {
    0 => 0.to_bigint().expect("invalid input"),
    1=> 1.to_bigint().expect("invalid input"),
  _ => fibonacci(c - 2) + fibonacci(c -1)
  }

}
