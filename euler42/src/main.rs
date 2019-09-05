// 借鉴22题
fn main() {
    // 准备足够的三角数
    let mut tri_numbers = vec![];
    for i in 1..100 {
        tri_numbers.push(i * (i+1) / 2);
    }
    println!("{:?}", tri_numbers);


    let data = std::fs::read_to_string("words.txt").expect("读文件失败");
    let data2 = remove_quote(&data);
    let names: Vec<&str> = data2.split(",").collect();

    let mut count = 0;
    for name in names {
        let ws = word_score(name);
        if tri_numbers.contains(&ws) {
            println!("{} {}", name, ws);
            count += 1;
        }
    }
    println!("\n{}", count);
}
// 162

//移除引号
fn remove_quote(s: &str) -> String {
    s.chars().filter(|c| *c != '"').collect()
}

// 单词在字母表中分数
fn word_score(word: &str) -> usize {
    let mut score = 0;
    for ch in word.chars() {
        score += letter_number(ch);
    }
    score
}

// 一个字符在字母表中分数
fn letter_number(ch: char) -> usize {
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    letters.chars().position(|c| c == ch).unwrap() + 1
}