use tests::lib::fibbonnacci;
fn main() {
    let y = 1;
 
   let args =   Args::parse();
   let v = args.args1;
   let u =args.args2;
  
if u == true && v >= y {
    let x = fibbonnacci(v, u, y);
    println!("The fibbonacci of {y} is: {:?}", x);
}

   
}
use clap::Parser;
#[derive(Parser)]
pub struct Args {

    args1: u128,
  
    args2: bool,
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
