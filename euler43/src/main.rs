// problem 8, 24
fn main() {
    let mut v = [1, 0, 2, 3, 4, 5, 6, 7, 8, 9];
    let primes = [0, 2, 3, 5, 7, 11, 13, 17];
    let mut sum: u64 = 0;
    loop {
        let v_str = v.iter().map(|x| x.to_string()).collect::<String>();
        //println!("{}", v_str);

        for i in 1..=7 {
            let sub3 = &v_str[i..i + 3];
            let d = sub3.parse::<u32>().unwrap();
            if d % primes[i] != 0 {
                break;
            }
            if i == 7 {
                println!("{}", v_str);
                sum += v_str.parse::<u64>().unwrap();
            }
        }

        if !next_perm(&mut v) {
            break;
        }
    }
    println!("sum: {}", sum);
}
// 16695334890

fn next_perm(v: &mut [u32]) -> bool {
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

fn swap(v: &mut [u32], i: usize, j: usize) {
    let temp = v[i];
    v[i] = v[j];
    v[j] = temp;
}
