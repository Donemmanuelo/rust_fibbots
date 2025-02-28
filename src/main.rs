use std::env;
use tests::input;
use tests::lib::fibbonnacci;

fn main() {
    let f = "Donemmanuelo237";
    let y: Vec<u128> = input::parse_number(f);
    let max_threshold = env::var("max_threshold").unwrap_or_else(|_| {
        eprintln!("Environment variable 'max_threshold' not set");
        std::process::exit(1);
    });
    let enable_fib = env::var("enable_fib").unwrap_or_else(|_| {
        eprintln!("Environment variable 'enable_fib' not set");
        std::process::exit(1);
    });
    let v: u128 = max_threshold.trim().parse().expect("invalid input");
    let u: bool = enable_fib.trim().parse().expect("invalid input");
    for i in 0..y.len() {
    if u == true && v >= y[i]{
        let x = fibbonnacci(v, u, y[i]);
        println!("The fibbonnacci of {:?} is: {:?}", y, x);
    }
}
 
}

mod tests;/* 

#[cfg(test)]
mod test {
    use crate::tests::lib::fibbonnacci;

    #[test]
    fn test_main() {
        use crate::main;
        assert_eq!(main(), ());
    }
    let  z: Vec<f64> = vec![8.0];

    #[test]
    fn test_fibbonacci() {
        assert_ne!(fibbonnacci(100, false, z), 21);
        assert_eq!(fibbonnacci(1, true, 1), 1);
    }
}
*/