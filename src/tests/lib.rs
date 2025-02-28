pub fn fibbonnacci(a: u128, b: bool, c: u128) -> u128 {

    let mut x: u128 = 0;
    let mut  y: u128 = 1;
    for i in 2..c {
        x = i - 2;
        y = i - 1; 
        x += y;
    }
x
    
}
