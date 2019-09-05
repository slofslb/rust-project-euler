fn main() {
    let max_number_to_check = 1_000_000;

    let mut prime_mask = vec![true; max_number_to_check];
    prime_mask[0] = false;
    prime_mask[1] = false;

    const FIRST_PRIME_NUMBER: usize = 2;
    for p in FIRST_PRIME_NUMBER..max_number_to_check {
        if prime_mask[p] {
            let mut i = 2 * p;
            while i < max_number_to_check {
                prime_mask[i] = false;
                i += p;
            }
        }
    }

    let mut sum = 0;
    for n in 10..max_number_to_check {
        if !prime_mask[n] {continue;}
        let mut s = n.to_string();

        let mut is_trun_prime = true;
        while { s.pop(); s.len() > 0} {
            let p = s.parse::<usize>().unwrap();
            if !prime_mask[p] {
                is_trun_prime = false;
                break;
            }
        }

        if !is_trun_prime { continue; }

        s = n.to_string();
        while s.len() > 1 {
            s.remove(0); 
            let p = s.parse::<usize>().unwrap();
            if !prime_mask[p] {
                is_trun_prime = false;
                break;
            }
        }
        if !is_trun_prime { continue; }
        println!("{}", n);
        sum += n;
        
    }
    println!("sum: {}", sum);
}
