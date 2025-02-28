
use std::env;

use tests::lib::fibbonnacci;
fn main() {
    let y = 1;
    let max_threshold = env::var("max_threshold").unwrap();
    let enable_fib = env::var("enable_fib").unwrap();
    let v: u128 = max_threshold.trim().parse().expect("invalid input");
    let u: bool = enable_fib.trim().parse().expect("invalid input");
  
if u == true && v >= y {
    let x = fibbonnacci(v, u, y);
    println!("The fibbonacci of {y} is: {:?}", x);
}
   
}
mod tests;
#[cfg(test)]
mod test {
    use crate::tests::lib::fibbonnacci;

    #[test]
    fn test_main() {
        use crate::main;
        assert_eq!(main(), ());
    }
    #[test]
    fn test_fibbonacci() {
        
        assert_ne!(fibbonnacci(100, false, 8), 21);
        assert_eq!(fibbonnacci(1, true, 1 ), 1);
    }
}