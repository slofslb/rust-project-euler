fn main() {
    let p3 = gen_poly_numbers(1000, 9999, |n| n * (n + 1) / 2);
    let p4 = gen_poly_numbers(1000, 9999, |n| n * n);
    let p5 = gen_poly_numbers(1000, 9999, |n| n * (3 * n - 1) / 2);
    let p6 = gen_poly_numbers(1000, 9999, |n| n * (2 * n - 1));
    let p7 = gen_poly_numbers(1000, 9999, |n| n * (5 * n - 3) / 2);
    let p8 = gen_poly_numbers(1000, 9999, |n| n * (3 * n - 2));

    for n in p8 {
        println!("{}", n);
        find(&[&p6, &p5, &p3, &p4, &p7], tail(n), head(n), &[]);
    }

}
//[8256, 5625, 2882, 8128, 2512, 1281]

fn head(n: u32) -> u32 {
    n / 100
}

fn tail(n: u32) -> u32 {
    n % 100
}

fn find(lists: &[&[u32]], start:u32, end:u32,found:&[u32]) {
    if lists.len() == 0 && tail(*found.last().unwrap()) == end {
        println!("found: {:?} {} {}", found, start, end);
    }

    for &cur_list in lists {
        for &n in cur_list {
            if head(n) == start {
                // remove cur_list
                let  lists_copy = &lists[1..];
                let mut found_new = found.to_vec();
                found_new.push(n);
                find(lists_copy, tail(n), end, &found_new);
                //println!("{:?}", lists_copy);
                //println!("{:?}", found_new);
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
