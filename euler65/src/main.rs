extern crate num_bigint;
use num_bigint::BigUint;

fn main() {
    // let mut (a, b) = (0, 1); // error
    let (mut a, mut b) = (BigUint::from(0 as u64), BigUint::from(1 as u64));
    for i in (1..=99).rev() {
        let e: u64 = if (i + 1) % 3 == 0 { (i + 1) / 3 * 2 } else { 1 };
        let temp = b.clone(); 
        b = &a + BigUint::from(e) * &temp;
        a = temp.clone();
        //(a, b) = (b.clone(), &a + BigUint::from(e) * b.clone()); // error
    }
    let temp = b.clone();
    a = &a + BigUint::from(2_u64) * &temp;
    b = temp.clone();
    println!("{:?} {:?}", a.to_string(), b.to_string());
    let sum = a
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>();
    println!("{}", sum);
}
