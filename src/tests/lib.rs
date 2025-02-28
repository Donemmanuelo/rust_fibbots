pub fn fibbonnacci(_a: f64, _b: bool, c: f64) -> u128 {

    let mut x: u128 = 0;
    let mut y: u128 = 1;
      let c = c as u128;
    for _ in 0..c {
     let temp = x + y ;
     x = y;
     y = temp;
 
    }
x
    
}
