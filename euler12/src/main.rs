fn main() {
    for i in 2.. {
        let triange_num = (1..=i).sum::<u32>();
        let f = factors(triange_num);
        if f.len() * 2 > 500 {
            println!("{}", triange_num);
            break;
        }
    }
}
// 76576500

// 只求一半的因子就行，可以大幅提高速度
fn factors(num: u32) -> Vec<u32> {
    let s = (num as f32).sqrt() as u32;
    (1..=s).filter(|x| num % x == 0).collect::<Vec<u32>>()
}
