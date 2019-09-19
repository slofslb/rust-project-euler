use std::collections::HashMap;
fn main() {
    let mut cubic = HashMap::new();
    for i in 10_u64..10000 {
        let pow3 = i.pow(3);
        let mut digits: Vec<u64> = pow3
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .collect();
        digits.sort();
        let key = digits.iter().rev().fold(0u64, |x, a| 10 * x + a);
        if !cubic.contains_key(&key) {
            cubic.insert(key, vec![i]);
        } else {
            let v = cubic.get_mut(&key).unwrap();
            &v.push(i);
            if v.len() == 5 {
                println!(
                    "{:?} {:?}",
                    v,
                    v.iter().map(|x| x * x * x).collect::<Vec<u64>>()
                );
                let min_v = v.iter().min().unwrap();
                println!("{}", min_v * min_v * min_v);
            }
        }
    }
    //println!("{:?}", cubic );
}
