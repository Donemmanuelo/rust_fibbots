use std::env;
use tests::input::extract_numbers;
use tests::lib::fibbonnacci;
use tests::value::bal;
use tests::comment::post_comment_to_pr;
fn main() {
    let f: &str = &bal();
    let numbers = extract_numbers(f);
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
    let owner = "Donemmanuelo"; 
    let repo = "rust_fibbots"; 
    let pr_number = 1;


  for i in 0..numbers.len() {
    if u == true && v >= numbers[i]{
        let x = fibbonnacci(v, u, numbers[i]);
        println!("The fibbonnacci of {:?} is: {:?}", numbers[i], x);

    if let Err(e) = post_comment_to_pr(owner, repo, pr_number, x) {
        eprintln!("Error posting comment: {}", e);
    }
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