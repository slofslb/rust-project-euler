fn main() {
    let mut count = 0;
    for n in 7..100 {
        let t = (n as f64) * 2.0_f64.log10();
        let m = t - t.floor() + 1.0;
        let m = 10_f64.powf(m).floor() as u64;
        if m == 12 {
            count += 1;
            println!("p(12, {}) = {} ", count, n);
            if count == 1 {
                assert_eq!(n, 7, "p(12,1) = 7");
            }
            if count == 2 {
                assert_eq!(n, 80, "p(12,2) = 80");
            }
        }
    }

    let mut count = 0;
    for n in 7.. {
        let t = (n as f64) * 2.0_f64.log10();
        let m = t - t.floor() + 2.0;
        let m = 10_f64.powf(m).floor() as u64;
        if m == 123 {
            count += 1;
            println!("p(123, {}) = {} ", count, n);
            if count == 678910 {
                break;
            }
        }
    }
}
// 193060223
