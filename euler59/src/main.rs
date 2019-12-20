fn main() {
    let data = std::fs::read_to_string("cipher.txt").expect("文件无法打开");
    let xs: Vec<&str> = data.split(",").collect();
    let letters: Vec<u8> = xs.iter().map(|x| x.parse::<u8>().unwrap()).collect();

    let key = [
        guess_pass(&letters, 0),
        guess_pass(&letters, 1),
        guess_pass(&letters, 2),
    ];
    println!("key: {:?}", key);

    // decode 
    let mut sum: u32 = 0;
    for (i, ch) in letters.iter().enumerate() {
        let a = ch ^ (key[i % 3] as u8);
        sum += a as u32;
        print!("{}", a as char);
    }
    println!("\n");
    println!("{}", sum);
}

// 用统计方法，尝试可能的密码，如果得到的明文中包含的英文字母越多，越可能是正确的密码
fn guess_pass(letters: &Vec<u8>, index: usize) -> char {
    let mut key = '*';
    let mut max_count = 0;
    for pass in ('a' as u8)..=('z' as u8) {
        // 统计次数
        let mut count = 0;
        for (i, ch) in letters.iter().enumerate() {
            if i % 3 == index {
                let a = (ch ^ pass) as char;
                if ('A'..='Z').contains(&a) || ('a'..='z').contains(&a) {
                    count += 1;
                }
            }
        }
        // 出现单词最多的，可能就是正确的密钥
        if count > max_count {
            max_count = count;
            key = pass as char;
        }
    }
    key
}
