use std::collections::HashMap;

fn main() {
    let mut min_cube = std::u64::MAX;
    let mut hash = HashMap::new();
    for i in 10_u64..10000 {
        let pow3 = i.pow(3);
        let mut digits: Vec<u64> = pow3
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .collect();
        digits.sort();
        // 反序排列之后的整数，作为哈希表的主键
        let key = digits.iter().rev().fold(0u64, |x, a| 10 * x + a);
        if !hash.contains_key(&key) {
            hash.insert(key, vec![i]);
        } else {
            let cubic_numbers = hash.get_mut(&key).unwrap();
            &cubic_numbers.push(i);
            if cubic_numbers.len() == 5 {
                println!("{:?}", cubic_numbers);
                println!("{:?}", cubic_numbers.iter().map(|x| x.pow(3)).collect::<Vec<u64>>());
                let temp = cubic_numbers.iter().min().unwrap().pow(3);
                if temp < min_cube {
                    min_cube = temp;
                }
            }
        }
    }
    println!("{}", min_cube);
}

// [5027, 7061, 7202, 8288, 8384]
// [127035954683, 352045367981, 373559126408, 569310543872, 589323567104]
// 127035954683 这个是最后的答案
// [5196, 8124, 8496, 9702, 9783]
// [140283769536, 536178930624, 613258407936, 913237656408, 936302451687]
// min: 140283769536
