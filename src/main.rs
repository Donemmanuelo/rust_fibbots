use tests::lib::output;

fn main() {
    let y = output();
    println!("{y}");
}
mod tests;
#[cfg(test)]
mod test {
    use crate::tests::lib::output;

    #[test]
    fn test_main() {
        use crate::main;
        assert_eq!(main(), ());
    }
    #[test]
    fn test_output() {
        
        assert_ne!(output(), "Hello, Rust!");
        assert_eq!(output(), "Hello, World!");
    }
}