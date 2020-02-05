use permutator::{cartesian_product};

fn main() {
    let dices_colin: &[&[u32]] = &[
        &[1, 2, 3, 4, 5, 6],
        &[1, 2, 3, 4, 5, 6],
        &[1, 2, 3, 4, 5, 6],
        &[1, 2, 3, 4, 5, 6],
        &[1, 2, 3, 4, 5, 6],
        &[1, 2, 3, 4, 5, 6],
    ];
    let mut count_colin = vec![0; 6 * 6 + 1];
    cartesian_product(dices_colin, |p| {
        let dice_sum = p.iter().map(|&&x| x).sum::<u32>();
        count_colin[dice_sum as usize] += 1;
        //println!("{:?} {}", p, dice_sum);
    });
    println!("{:?}", count_colin);

    let dices_peter: &[&[u32]] = &[
        &[1, 2, 3, 4],
        &[1, 2, 3, 4],
        &[1, 2, 3, 4],
        &[1, 2, 3, 4],
        &[1, 2, 3, 4],
        &[1, 2, 3, 4],
        &[1, 2, 3, 4],
        &[1, 2, 3, 4],
        &[1, 2, 3, 4],
    ];
    let mut count_peter = vec![0; 4 * 9 + 1];
    cartesian_product(dices_peter, |p| {
        let dice_sum = p.iter().map(|&&x| x).sum::<u32>();
        count_peter[dice_sum as usize] += 1;
        //println!("{:?} {}", p, dice_sum);
    });
    println!("{:?}", count_peter);

    let mut prob = 0.0;
    // 色子和最小为6，最大36
    for dice_sum in 6..=36 {
        let prob1 = count_colin[dice_sum] as f64 / 6_f64.powf(6.0);
        // 假设第1组色子扔到6，第2组色子的点数和必须大于6，把这些可能性加起来
        let prob2 = count_peter[dice_sum + 1..=36].iter().sum::<u64>() as f64 / 4_f64.powf(9.0);
        prob += prob1 * prob2;
    }
    println!("{:.7}", prob);
}
// 0.5731441

/*
// https://stackoverflow.com/questions/50277050/is-there-a-built-in-function-that-converts-a-number-to-a-string-in-any-base
fn format_radix(mut x: u32, radix: u32, mut length: u32) -> String {
    let mut result = vec![];

    while length > 0 {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix).unwrap());
        length -= 1;
        // if x == 0 {
        //     break;
        // }
    }
    result.into_iter().rev().collect()
}

// 扔几个色子，在所有可能性中，各种点数之和出现的次数
// 例如：如果返回[0, 1, 4, 8 ... ]
// 表示，点数和为0的出现0次，和为1的出现1次，和为2的出现4次，和为3的出现8次。
fn dice_sum_count(face: u32, dice_num: u32) -> Vec<u64> {
    let mut v = vec![0; (face * dice_num + 1) as usize];
    for n in 0..face.pow(dice_num) {
        let str = format_radix(n, face, dice_num);
        let dice_sum = str
            .chars()
            .map(|c| c.to_digit(10).unwrap() + 1)
            .sum::<u32>();
        v[dice_sum as usize] += 1;
    }
    v
}
*/