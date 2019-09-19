    use std::collections::HashMap;
fn main() { 
    let mut cubic = HashMap::new();
    for i in 100_u32..1000 {
        let pow3 = i.pow(3);
        let mut digits :Vec<u32> = pow3.to_string().chars().map(|x| x.to_digit(10).unwrap()).collect();
        digits.sort();
        let key = digits.iter().rev().map(|x| x.).collect::<String>();
        //let key = digits.iter().rev().map(|x| x * x * x).sum::<u32>();
        if !cubic.contains_key(&key) {
            cubic.insert(key, i);
        }
        else {
            println!("{} {} key:{}", i, pow3, key);
        }
    }
    println!("{:?}", cubic );
    
}
