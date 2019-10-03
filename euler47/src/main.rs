// 有许多重复计算，没做优化
fn main() {
    for n in 2.. {
        if meet_cond(n) {
            println!("{}", n);
            break;
        }
    }
    // 另一种写法
    let n = (2..).filter(|x| meet_cond(*x)).take(1).next().unwrap();
    println!("{:?}", n);
}
// 134043

fn meet_cond(n: u64) -> bool {
    let xf = primes::factors_uniq(n);
    if xf.len() != 4 {
        return false;
    }
    for i in 1..=3 {
        let yf = primes::factors_uniq(n + i);
        if yf.len() != 4 || xf == yf {
            return false;
        }
    }
    true
}
