use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();
    println!("{}", summation_ways(100, 99));
    println!("{:?}", start.elapsed());

    // 第二种解法：
    let start = SystemTime::now(); 
    let mut cache = vec![0_i64; 101];
    println!("{}", p(&mut cache, 100) - 1);
    println!("{:?}", start.elapsed());
}
// 190569291

fn summation_ways(s: u32, limit: u32) -> u32 {
    if limit <= 1 {
        return 1;
    }
    let mut count = 0;
    for i in 1..=limit {
        count += summation_ways(s - i, i.min(s - i));
    }
    //println!("{} {} {}", s, limit, count);
    count
}

// http://mathworld.wolfram.com/PartitionFunctionP.html
// http://oeis.org/A000041
// 与参考文献的稍有不同，P(0) = 1 ，P(<0) = 0
fn p(cache: &mut [i64], n: i64) -> i64 {
    if n < 0 {
        return 0;
    }
    if n == 0 {
        return 1;
    }
    if cache[n as usize] > 0 {
        return cache[n as usize];
    }
    let mut s = 0;
    for k in 1..=n {
        let n1 = n - k * (3 * k - 1) / 2;
        let n2 = n - k * (3 * k + 1) / 2;
        let sign = if (k + 1) % 2 == 0 { 1 } else { -1 };
        s += sign * (p(cache, n1) + p(cache, n2));
    }
    cache[n as usize] = s;
    s
}
