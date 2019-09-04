// outer break
// 两重循环有问题
fn main() {
    let mut max = 0;
    for x in 100..=999 {
        for y in 100..=999 {
            let prod = x * y;
            if is_palindromic(prod) && prod > max {
                max = prod;
                println!("{} x {} = {}", x, y, prod);
            }
        }
    }
    println!("{}", max);
}

/*
fn main() {
    'outer: for x in (100..=999).rev() {
        for y in (x..=999).rev() {
            let prod = x * y;
            if is_palindromic(prod) {
                println!("{} x {} = {}", x, y, prod);
                break 'outer;
            }
        }
    }
}
*/
// 924 x 962 = 888888
/* error
fn main() {
    let mut x = 999;
    let mut y = 999;
    while x > 100 && y > 100 {
        let prod = x * y;
        println!("{} x {}", x, y);

        if is_palindromic(prod) {
            println!("{} x {} = {}", x, y, prod);
            break;
        }

        if x >= y {
            x -= 1;
        } else {
            y -= 1;
        }
    }
}
// 836 x 836 = 698896
*/

/* // error
#[macro_use]
extern crate itertools;
fn main() {
    let a = (100..999).rev();
    let b = (100..999).rev();
    for (x, y) in iproduct!(a, b) {
        let prod = x * y;
        println!("{} x {}", x, y);

        if is_palindromic(prod) {
            println!("{} x {} = {}", x, y, prod);
            break;
        }
    }
}
*/

/* //error
fn main() {
    'outer: for x in (100..=999).rev() {
        for y in (100..=999).rev() {
            let prod = x * y;
            if is_palindromic(prod) {
                println!("{} x {} = {}", x, y, prod);
                break 'outer;
            }
        }
    }
}
// 836 x 836 = 698896
*/

fn is_palindromic(n: u64) -> bool {
    let s = n.to_string();
    s.chars().rev().collect::<String>() == s
}
