fn main() {
    let data = std::fs::read_to_string("names.txt").expect("读文件失败");
    let data2 = remove_quote(&data);
    let mut names: Vec<&str> = data2.split(",").collect();

    names.sort();

    let mut score = 0;
    for (i, name) in names.iter().enumerate() {
        let ws = word_score(name);
        println!("{} {} {}", (i + 1), name, ws);
        score += ws * (i + 1);
    }
    println!("{}", score);
}

fn remove_quote(s: &str) -> String {
    s.chars().filter(|c| *c != '"').collect()
}

fn letter_number(ch: char) -> usize {
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    letters.chars().position(|c| c == ch).unwrap() + 1
}

fn word_score(word: &str) -> usize {
    let mut score = 0;
    for ch in word.chars() {
        score += letter_number(ch);
    }
    score
}
