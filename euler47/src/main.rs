//use primes::PrimeSet;

fn main() {
    // 有许多重复计算，没做优化
    for n in 2.. {
        let af = primes::factors_uniq(n);
        let bf = primes::factors_uniq(n + 1);
        let cf = primes::factors_uniq(n + 2);
        let df = primes::factors_uniq(n + 3);
        if af.len() == 4
            && bf.len() == 4
            && cf.len() == 4
            && df.len() == 4
            && af != bf
            && af != cf
            && af != df
            && bf != cf
            && bf != df
            && cf != df
        {
            println!("{} {:?} {:?} {:?} {:?}", n, af, bf, cf, df);
            break;
        }
    }
}
