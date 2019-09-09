fn main() {
    let data = std::fs::read_to_string("cipher.txt").expect("文件无法打开");

    let xs: Vec<&str> = data.split(",").collect();
    let letters: Vec<u8> = xs.iter().map(|x| x.parse::<u8>().unwrap()).collect();

    // 用统计方法，尝试可能的密码，如果得到的明文中包含的英文字母越多，越可能是正确的密码。
    let mut pass = ['.'; 3];
    let mut max_count = [0; 3];
    for p in 0x61..=0x7A { // 密码是小写字母
        let mut count = [0; 3];
        for (i, ch) in letters.iter().enumerate() {
            let a = (ch ^ p) as char;
            if ('A'..='Z').contains(&a) || ('a'..='z').contains(&a) {
                count[i % 3] += 1;
            }
        }
        for i in 0..3 {
            if count[i] > max_count[i] {
                max_count[i] = count[i];
                pass[i] = p as char;
            }
        }
    }
    println!("key: {:?}", pass);

    let mut sum: u32 = 0;
    for (i, ch) in letters.iter().enumerate() {
        let a = ch ^ (pass[i % 3] as u8);
        sum += a as u32;
        print!("{}", a as char);
    }
    println!("\n{}", sum);
}
