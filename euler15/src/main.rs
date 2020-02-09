fn main() {
    let mut v: Vec<u64> = vec![0; 10000];
    println!("{}", path(&mut v, 20, 20));
    println!("12 x 12: slow {}", path_slow(12, 12));
}
// 137846528820

fn path(v: &mut Vec<u64>, m: usize, n: usize) -> u64 {
    if m == 0 || n == 0 {
        return 1;
    }
    if v[m * 100 + n] > 0 {
        return v[m * 100 + n];
    } //缓存命中
    let mut sum = 0;
    for j in 0..=n {
        sum += path(v, m - 1, j);
    }
    v[m * 100 + n] = sum; // 加入缓存中
    println!("({},{}) {}", m, n, sum);
    sum
}

fn path_slow(m: usize, n: usize) -> u64 {
    if m == 0 || n == 0 {
        return 1;
    }
    let mut sum = 0;
    for j in 0..=n {
        sum += path_slow(m - 1, j);
    }
    sum
}
