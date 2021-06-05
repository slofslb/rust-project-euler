fn main() {
    let mut fib = vec![1, 2];
    // let mut fib = Vec::new();
    // fib.push(1);
    // fib.push(2);

    let mut i = 2;
    let mut sum = 2;
    loop {
        let c = fib[i - 1] + fib[i - 2];
        if c >= 4_000_000 {
            break;
        }
        fib.push(c);
        if c % 2 == 0 {
            sum += c;
        }
        i += 1;
    }
    println!("{}", sum);

    // 另一种写法
    let mut fib = vec![1, 2];
    for i in 2.. {
        let c = fib[i - 1] + fib[i - 2];
        if c >= 4_000_000 {
            break;
        }
        fib.push(c);
    }
    println!("{}", fib.iter().filter(|&x| x % 2 == 0).sum::<u32>());
}

/*
#![feature(phase)]
#[phase(plugin, link)]
//extern crate grabbag_macros;

static LIMIT: u64 = 4000000;

fn main() {
    let sum = recurrence![f[n]: u64 = 1, 1... f[n-1] + f[n-2]]
        .take_while(|&n| n <= LIMIT)
        .filter(|&n| n % 2 == 0)
        .fold(0, |a, b| a + b);
    println!("Sum of fibs: {}", sum);
}
*/

// 4613732
/*
https://github.com/gifnksm/ProjectEulerRust/blob/master/src/bin/p002.rs
extern crate common;
extern crate num_integer;
extern crate seq;

use num_integer::Integer;
use seq::Fibonacci;

fn compute(bound: u32) -> u32 {
    Fibonacci::<u32>::new()
        .take_while(|&f| f < bound)
        .filter(|&f| f.is_even())
        .sum()
}

fn solve() -> String {
    compute(4000000).to_string()
}
problem!("4613732", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn sum_below_90() {
        assert_eq!(44, super::compute(90));
    }
}

*/
