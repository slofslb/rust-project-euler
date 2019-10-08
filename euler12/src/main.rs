fn main() {
    for i in 2.. {
        let triange_num = (1..=i).sum::<u32>();
        let f = factors(triange_num);
        if f.len() * 2 > 500 {
            println!("{}", triange_num);
            break;
        }
    }

    // 另一种写法
    println!(
        "{}",
        (2..)
            .map(|i| (1..=i).sum::<u32>())
            .filter(|&x| factors(x).len() * 2 > 500)
            .next()
            .unwrap()
    );

    // 也可以这样写
    println!(
        "{}",
        (2..)
            .map(|i| (1..=i).sum::<u32>())
            .find(|&x| factors(x).len() * 2 > 500)
            .unwrap()
    );
}
// 76576500

// 只求一半的因子就行，可以大幅提高速度
fn factors(num: u32) -> Vec<u32> {
    let s = (num as f32).sqrt() as u32;
    (1..=s).filter(|x| num % x == 0).collect::<Vec<u32>>()
}
