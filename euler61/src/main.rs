fn main() {
    let p3 = gen_poly_numbers(1000, 9999, |n| n * (n + 1) / 2);
    let p4 = gen_poly_numbers(1000, 9999, |n| n * n);
    let p5 = gen_poly_numbers(1000, 9999, |n| n * (3 * n - 1) / 2);
    let p6 = gen_poly_numbers(1000, 9999, |n| n * (2 * n - 1));
    let p7 = gen_poly_numbers(1000, 9999, |n| n * (5 * n - 3) / 2);
    let p8 = gen_poly_numbers(1000, 9999, |n| n * (3 * n - 2));

    for &n in &p5 {
        find(&[&p3, &p4], tail(n), head(n), &[]);
    }

    for n in p8 {
        find(&[&p3, &p4, &p5, &p6, &p7], tail(n), head(n), &[]);
    }
}
// [8128, 2882, 8256, 5625, 2512, 1281] 28684

fn find(lists: &[&[u32]], start: u32, end: u32, found: &[u32]) {
    if lists.is_empty() && tail(*found.last().unwrap()) == end {
        let mut result = found.to_vec();
        result.push(end * 100 + head(found[0]));
        println!("{:?} {}", result, result.iter().sum::<u32>())
    }

    for (i, &cur_list) in lists.iter().enumerate() {
        for &n in cur_list {
            if head(n) == start {
                let mut lists_copy = lists.to_vec();
                lists_copy.remove(i);
                let mut found_copy = found.to_vec();
                found_copy.push(n);
                find(&lists_copy, tail(n), end, &found_copy);
                //println!("{:?} {:?}", lists_copy, found_new);
            }
        }
    }
}

fn gen_poly_numbers(start: u32, end: u32, f: fn(u32) -> u32) -> Vec<u32> {
    (1..)
        .map(f)
        .filter(|&n| n >= start)
        .take_while(|&n| n <= end)
        .collect()
}

fn head(n: u32) -> u32 {
    n / 100
}

fn tail(n: u32) -> u32 {
    n % 100
}
