fn main() {
    let mut dist = std::i64::MAX;
    for m in 1..100 {
        for n in 1..=m {
            let rects = r(m, n);
            let temp = (rects - 2_000_000_i64).abs();
            if temp < dist {
                println!("{} x {}  area={} rects={}", m, n, m * n, rects);
                dist = temp;
            }
        }
    }
}
// 77 x 36  area=2772 rects=1999998

fn r(m: i64, n: i64) -> i64 {
    if n == 1 {
        return m * (m + 1) / 2;
    }
    return r(m, n - 1) + m * (m + 1) / 2 * n;
}
