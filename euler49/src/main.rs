use permutohedron::heap_recursive;

fn main() {
    let max_number_to_check = 10000;
    let mut prime_mask = vec![true; max_number_to_check];
    fill_prime_mask(&mut prime_mask);

    for x in 1000..9999 {
        if !prime_mask[x] {
            continue;
        }
        let mut vx: Vec<u32> = x
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        //println!("{:?}", vx);

        let mut ys = Vec::new();
        heap_recursive(&mut vx, |pt| {
            let perm_x = pt
                .to_vec()
                .iter()
                .map(|p| p.to_string())
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
            ys.push(perm_x)
        });
        //println!("{:?}", ys);

        for y in ys.iter() {
            let diff = (*y as i32) - (x as i32);
            if diff <= 0 {
                continue;
            };
            if !prime_mask[*y as usize] {
                continue;
            }
            let z = diff as u32 + y;
            if ys.contains(&z) && prime_mask[z as usize] {
                println!("{} {} {}", x, y, z);
            }
        }
    }
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
