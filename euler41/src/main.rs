fn main() {
    let mut v = [7, 6, 5, 4, 3, 2, 1];
    loop {
        let d = v.iter().fold(0, |x, a| 10 * x + a);
        if primes::is_prime(d as u64) {
            println!("{}", d);
            break;
        }
        if !next_perm_desc(&mut v) {
            break;
        }
    }
}
// 7652413
// 降序全排列
fn next_perm_desc(v: &mut [u64]) -> bool {
    let mut i = v.len() - 2;
    while v[i] < v[i + 1] {
        if i == 0 {
            return false;
        }
        i -= 1;
    }

    let mut j = v.len() - 1;
    while i < j && v[i] < v[j] {
        j -= 1;
    }

    v.swap(i, j);

    i += 1;
    j = v.len() - 1;
    while i < j {
        v.swap(i, j);
        i += 1;
        j -= 1;
    }
    true
}

/*
// 借鉴第24题，再改为依赖第三方的排列组合库来实现
use permutohedron::heap_recursive;

fn main() {
    let mut max_prime = 0;
    let mut data = [1, 2, 3, 4, 5, 6, 7];
    heap_recursive(&mut data, |permutation| {
        let v = permutation.to_vec();
        // println!("{:?}", v);
        // vec![1, 2, 3, 4, 5, 6, 7] -> 1234567
        let d = v.iter().fold(0, |x, a| 10 * x + a);
        if primes::is_prime(d as u64) && d > max_prime {
            println!("{}", d);
            max_prime = d;
        }
    })
}
*/
