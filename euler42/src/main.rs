// 借鉴22题
// 1）读文件里的单词，放到一个数组里
// 2) 计算一个单词的分值
fn main() {
    // 准备足够的三角数
    let mut tri_numbers = vec![];
    for i in 1..100 {
        tri_numbers.push(i * (i + 1) / 2);
    }
    println!("{:?}", tri_numbers);

    let data = std::fs::read_to_string("words.txt").expect("读文件失败");
    let data2: String = data.chars().filter(|c| *c != '"').collect();
    let names: Vec<&str> = data2.split(",").collect();

    let mut count = 0;
    for name in names {
        let word_score = name.chars().map(|ch| ch as usize - 64).sum();
        if tri_numbers.contains(&word_score) {
            println!("{} {}", name, word_score);
            count += 1;
        }
    }
    println!("\n{}", count);
}
// 162

//移除引号
/*
fn remove_quote(s: &str) -> String {
    s.chars().filter(|c| *c != '"').collect()
}
*/

// 单词在字母表中分数
//fn word_score(word: &str) -> usize {
//    word.chars().map(|ch| ch as usize - 64).sum()
/*let mut score = 0;
for ch in word.chars() {
    score += letter_number(ch) as usize;
}
score */
//}

/*
// 一个字符在字母表中分数，'A' -> 1，'B' -> 2
fn letter_number(ch: char) -> u8 {
    // 字母A的ASCII编码为65
    (ch as u8) - 64
    //let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    //letters.chars().position(|c| c == ch).unwrap() + 1
}
*/
