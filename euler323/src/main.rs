fn main() {
    let mut sum = 0 as f64;
    for count in 1.. {
        sum += trial() as f64;
        println!("{}", sum / count as f64);
    }
    //println!("{}", sum / 1000000 as f64);
}

fn trial() -> u32 {
    let mut x = 0_u32;
    for i in 0.. {
        x = x | rand::random::<u32>();
        //println!("{} {:X}", i, x);
        if x == 0xFFFFFFFF {
            //println!("N: {}", i);
            return i;
        }
    }
    0
}
// 经过这个简单的测试，大概期望值为5.35左右
