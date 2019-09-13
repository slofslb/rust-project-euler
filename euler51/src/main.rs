//use primes::PrimeSet;

fn main() {
    let start = 121313_usize;
    let end = 999999;

    let max_number_to_check = end + 1;
    let mut prime_mask = vec![true; max_number_to_check];
    fill_prime_mask(&mut prime_mask);

    let mut vp = vec![];
    for p in start..end {
        if prime_mask[p] {
            vp.push(p);
        }
    }
    println!("prime len: {}", vp.len() );

    let mut count: Vec<usize> = vec![0; end];
    'exit: for i in 0..end.to_string().len() {
        for j in i + 1..end.to_string().len() {
            for i in 0..count.len() {
                count[i] = 0;
            }
            for p in &vp {
                if *p < start {continue;}
                let (a, digit_removed1) = remove(*p, i);
                let (b, digit_removed2) = remove(a, j);
                if digit_removed1 == digit_removed2 {
                    count[b] += 1;
                }
            }
            //println!("{:?}", count );

            for p in start..end {
                if count[p] >= 8 {
                    println!("{:?} {} {}", p, i, j);
                    break 'exit;
                }
            }
        }
    }
}

static POWER :&[usize] = &[1, 10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000, 
               100_000_000, 1000_000_000, 10_000_000_000];

fn remove(n: usize, i: usize) -> (usize, usize) {
    let d = n / POWER[i] % 10;
    (n - d * POWER[i], d)
}

fn fill_prime_mask(prime_mask: &mut [bool]) {
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
