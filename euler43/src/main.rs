// problem 8, 24
fn main() {
    let mut v = [0, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    let mut sum: u64 = 0;
    while next_perm(&mut v) {
        let num = v.iter().fold(0, |x, a| 10 * x + a);
        if is_divisibility(num) {
            println!("{}", num);
            sum += num;
        }
    }
    println!("sum: {}", sum);
}
// 16695334890

fn is_divisibility(num: u64) -> bool {
    let primes = [2, 3, 5, 7, 11, 13, 17];
    let mut n = num % 1_000_000_000;
    for i in (0..=6).rev() {
        let sub3 = n % 1000;
        if sub3 % primes[i] != 0 {
            return false;
        }
        n = n / 10;
    }
    true
}

fn next_perm(v: &mut [u64]) -> bool {
    let mut i = v.len() - 2;
    while v[i] > v[i + 1] {
        if i == 0 {
            return false;
        } // 已经全部从大到小排列了
        i -= 1;
    }

    let mut j = v.len() - 1;
    while i < j && v[i] > v[j] {
        j -= 1;
    }

    swap(v, i, j);

    i += 1;
    j = v.len() - 1;
    while i < j {
        swap(v, i, j);
        i += 1;
        j -= 1;
    }
    true
}

fn swap(v: &mut [u64], i: usize, j: usize) {
    let temp = v[i];
    v[i] = v[j];
    v[j] = temp;
}
