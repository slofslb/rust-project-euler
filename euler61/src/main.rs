fn main() {
    let p3 = gen_poly_numbers(1000, 9999, |n| n * (n + 1) / 2);
    let p4 = gen_poly_numbers(1000, 9999, |n| n * n);
    let p5 = gen_poly_numbers(1000, 9999, |n| n * (3 * n - 1) / 2);
    let p6 = gen_poly_numbers(1000, 9999, |n| n * (2 * n - 1));
    let p7 = gen_poly_numbers(1000, 9999, |n| n * (5 * n - 3) / 2);
    let p8 = gen_poly_numbers(1000, 9999, |n| n * (3 * n - 2));

    for n in p3 {
        find(&[&p4, &p5, &p6, &p7, &p8], tail(n), head(n), &[n]);
    }
}
// [8128, 2882, 8256, 5625, 2512, 1281] 28684

#[test]
fn test3() {
    let p3 = gen_poly_numbers(1000, 9999, |n| n * (n + 1) / 2);
    let p4 = gen_poly_numbers(1000, 9999, |n| n * n);
    let p5 = gen_poly_numbers(1000, 9999, |n| n * (3 * n - 1) / 2);
    let mut result = 0;
    for n in p3 {
        result = find(&[&p4, &p5], tail(n), head(n), &[n]);
        if result > 0 {
            break;
        }
    }
    // [8128, 2882, 8281] 19291
    assert_eq!(result, 19291);
}

fn find(lists: &[&[u32]], start: u32, end: u32, found: &[u32]) -> u32 {
    if lists.is_empty() && start == end {
        let result = found.iter().sum::<u32>();
        println!("{:?} {}", found, result);
        return result;
    }

    for (i, &cur_list) in lists.iter().enumerate() {
        for &n in cur_list {
            if head(n) == start {
                let mut lists_copy = lists.to_vec();
                lists_copy.remove(i);
                let mut found_copy = found.to_vec();
                found_copy.push(n);
                let result = find(&lists_copy, tail(n), end, &found_copy);
                if result > 0 {
                    return result;
                }
                //println!("{:?} {:?}", lists_copy, found_new);
            }
        }
    }
    0
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
