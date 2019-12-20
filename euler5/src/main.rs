fn main() {
    'outer: for x in (100..).step_by(2) {
        for f in (2..=20).rev() {
            if x % f != 0 {
                break;
            }
            if f == 2 {
                println!("{}", x);
                break 'outer;
            }
        }
    }

    // 另一种写法
    for x in (100..).step_by(2) {
        if can_divide_1_to_20(x) {
            println!("{}", x);
            break;
        }
    }

    // 还可以这样写
    print!("{}", (100..).step_by(2).filter(|&x| can_divide_1_to_20(x)).next().unwrap());
}
// 232792560

fn can_divide_1_to_20(x: u64) -> bool {
    for f in (2..=20).rev() {
        if x % f != 0 {
            return false;
        }
    }
    true
}
