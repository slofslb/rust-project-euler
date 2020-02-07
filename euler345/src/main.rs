fn main() {
    let mut cache = vec![0; 2_usize.pow(5)];
    let matrix = vec![
        7, 53, 183, 439, 863, // row 1
        497, 383, 563, 79, 973, // row 2
        287, 63, 343, 169, 583, // row 3
        627, 343, 773, 959, 943, // row 4
        767, 473, 103, 699, 303,
    ];
    let mut p = vec![];
    assert_eq!(3315, matrix_sum(&mut cache, &matrix, &mut p));

    let mut cache = vec![0; 2_usize.pow(15)];
    let matrix = vec![
        7, 53, 183, 439, 863, 497, 383, 563, 79, 973, 287, 63, 343, 169, 583, 627, 343, 773, 959,
        943, 767, 473, 103, 699, 303, 957, 703, 583, 639, 913, 447, 283, 463, 29, 23, 487, 463,
        993, 119, 883, 327, 493, 423, 159, 743, 217, 623, 3, 399, 853, 407, 103, 983, 89, 463, 290,
        516, 212, 462, 350, 960, 376, 682, 962, 300, 780, 486, 502, 912, 800, 250, 346, 172, 812,
        350, 870, 456, 192, 162, 593, 473, 915, 45, 989, 873, 823, 965, 425, 329, 803, 973, 965,
        905, 919, 133, 673, 665, 235, 509, 613, 673, 815, 165, 992, 326, 322, 148, 972, 962, 286,
        255, 941, 541, 265, 323, 925, 281, 601, 95, 973, 445, 721, 11, 525, 473, 65, 511, 164, 138,
        672, 18, 428, 154, 448, 848, 414, 456, 310, 312, 798, 104, 566, 520, 302, 248, 694, 976,
        430, 392, 198, 184, 829, 373, 181, 631, 101, 969, 613, 840, 740, 778, 458, 284, 760, 390,
        821, 461, 843, 513, 17, 901, 711, 993, 293, 157, 274, 94, 192, 156, 574, 34, 124, 4, 878,
        450, 476, 712, 914, 838, 669, 875, 299, 823, 329, 699, 815, 559, 813, 459, 522, 788, 168,
        586, 966, 232, 308, 833, 251, 631, 107, 813, 883, 451, 509, 615, 77, 281, 613, 459, 205,
        380, 274, 302, 35, 805,
    ];
    println!("{}", matrix_sum(&mut cache, &matrix, &mut p));

    // ugly to find the positions!
    let mut p = vec![];
    for j in 0..15 {
        let mut index = 0;
        let mut mmax = 0;
        for i in 0..15_usize {
            if !p.contains(&i) {
                p.push(i);
                let hash = hash_code(&p);
                if cache[hash] == 0 {
                    println!("no cache");
                    index = i;
                }
                if cache[hash] > mmax {
                    mmax = cache[hash];
                    index = i;
                    //println!("{:?} {}", p, cache[hash]);
                }
                p.pop();
            }
        }
        p.push(index);
        println!("{:?}", p);
    }
}
// 13938

fn matrix_sum(cache: &mut [usize], v: &Vec<usize>, p: &mut Vec<usize>) -> usize {
    let r = (v.len() as f64).sqrt() as usize;
    let col = p.len();
    if col == r {
        return 0;
    }

    let hash = hash_code(p);
    //println!("{:?} {}", &p, hash);
    if cache[hash] > 0 {
        if p.len() < 2 {
            //println!("{:?}", p);
        }
        return cache[hash];
    }

    let mut max_sum = 0;
    for row in 0..r {
        let index = row * r + col;
        if !p.contains(&row) {
            let mut p2 = p.clone();
            p2.push(row);
            let temp = v[index] + matrix_sum(cache, &v, &mut p2);
            if temp > max_sum {
                max_sum = temp;
            }
        }
    }
    cache[hash] = max_sum;
    //println!("{:?} {} {}", &p, hash, max_sum);
    max_sum
}

fn hash_code(p: &Vec<usize>) -> usize {
    let mut hash = 0;
    for element in p.iter() {
        hash += 1 << *element;
    }
    hash
}
