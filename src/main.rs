
use tests::lib::fibbonnacci;

fn main() {
    let y = 100;
    let max_threshold = std::env::var("1000").expect("max_threshold should have a value");
    let enable_fib = std::env::var("true").expect("enable_fib should have a value");
    println!("{y}");
    let v: u128 = max_threshold.trim().parse().expect("invalid input");
    let u: bool = enable_fib.trim().parse().expect("invalid input");
 
if u == true && y <= v {
    let x = fibbonnacci(v, u, y);
    println!("The fibbonacci of y is: {:?}", x);
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