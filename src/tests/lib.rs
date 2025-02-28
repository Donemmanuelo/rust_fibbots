use num_bigint::BigInt;
use num_bigint::ToBigInt;

pub fn fibbonnacci(_a: u128, _b: bool, c: u128) -> BigInt{

    let mut x: BigInt = 0.to_bigint().expect("REASON");
    let mut y: BigInt = 1.to_bigint().expect("REASON");
      let c = c as u128;
    for _ in 0..c {
     let temp = x + &y ;
     x = y;
     y = temp;
 
    }
    x.to_bigint().expect("REASON")
}
