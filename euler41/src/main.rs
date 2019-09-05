// 借鉴第24题和37题
fn main() {
    let max_number_to_check = 7_654_321 + 1;
    let mut prime_mask = vec![true; max_number_to_check];
    fill_prime_mask(&mut prime_mask);

    let mut v = [1, 2, 3, 4, 5, 6, 7];
    let mut max_prime = 0;
    loop {
        next_perm(&mut v);
        let v_str = v.iter().map(|x| x.to_string()).collect::<String>();
        let d = v_str.parse::<usize>().unwrap();
        if d >= 7_654_321 {
            break;
        }
        if prime_mask[d] && d > max_prime {
            println!("{}", d);
            max_prime = d;
        }
    }
}
// 7652413

fn next_perm(v: &mut [u32]) {
    let mut i = v.len() - 2;
    while v[i] > v[i + 1] {
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
}

fn swap(v: &mut [u32], i: usize, j: usize) {
    let temp = v[i];
    v[i] = v[j];
    v[j] = temp;
}

fn fill_prime_mask(prime_mask: &mut Vec<bool>) {
    prime_mask[0] = false;
    prime_mask[1] = false;

    const FIRST_PRIME_NUMBER: usize = 2;
    for p in FIRST_PRIME_NUMBER..prime_mask.len() {
        if prime_mask[p] {
            let mut i = 2 * p;
            while i < prime_mask.len() {
                prime_mask[i] = false;
                i += p;
            }
        }
    }
}
