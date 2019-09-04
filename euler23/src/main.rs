// 可以借鉴21题的函数, 发现了21题的一个bug
fn main() {
    let mut abundant_numbers = vec![false; 28124];
    for i in 2usize..abundant_numbers.len() {
        if is_abundant_number(i as u32) {
            abundant_numbers[i] = true;
            //println!("{}", i);
        }
    }

    let all_abundants = (1..=28123).filter(|&x| is_abundant_number(x)).collect::<Vec<u32>>();
    println!("{:?}", all_abundants);

    let mut sum = 0;
    for i in 1..=28123 {
        if !can_divide(&abundant_numbers, i) {
            sum += i;
        }
    }
    println!("sum: {}", sum);

    println!("sum: {}", (1..=28123).filter(|&x| !can_divide(&abundant_numbers, x)).sum::<u32>());

}
// 4179871

fn can_divide(abundant_numbers: &[bool], num: u32) -> bool {
    for x in 1..=28123 {
        let y = num - x;
        if y <= 0 {break;} 
        if abundant_numbers[x as usize] && abundant_numbers[y as usize] {
            // println!("can_divide: {} = {} + {}", num, x, y);
            return true;
        }
    }
    return false;
}


fn proper_divisors(num: u32) -> Vec<u32> {
    let mut v = { // 求一半的因子
        let s = (num as f32).sqrt() as u32;
        (1..=s).filter(|x| num % x == 0).collect::<Vec<u32>>()
    };
    let last = v.last().unwrap();
    if last * last == num {
        // 16的一半因子为1,2,4，另外只差一个8，即16 / 2
        for i in (1..v.len()-1).rev() {
            v.push(num / v[i]);
        }
    }
    else {
        // 12的一半因子为1,2,3，另外一半因子：4,6，分别对应于12/3，12/2
        for i in (1..v.len()).rev() {//不要num自身，所以从1开始
            v.push(num / v[i]);
        }
    }
    v
}


fn is_abundant_number(num: u32) -> bool {
    //println!("{:?}", proper_divisors(num));
    let proper_divisors_sum = proper_divisors(num).iter().sum::<u32>();
    proper_divisors_sum > num 
}
