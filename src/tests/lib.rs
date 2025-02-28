pub fn fibbonnacci(_a: u128, _b: bool, c: u128) -> u128 {

    let mut x: u128 = 0;
    let mut y: u128 = 1;
      
    for _ in 0..c  {
     let temp = x + y ;
     x = y;
     y = temp;
 
    }
x
    
}
