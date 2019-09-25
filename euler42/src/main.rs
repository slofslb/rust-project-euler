// 借鉴22题
// 1）读文件里的单词，放到一个数组里
// 2) 计算一个单词的分值
fn main() {
    // 准备足够的三角数
    let mut tri_numbers = vec![];
    for i in 1..=100 {
        tri_numbers.push(i * (i + 1) / 2);
    }
    //println!("{:?}", tri_numbers);

    let data = std::fs::read_to_string("words.txt").expect("读文件失败");
    // 删除引号
    let data2: String = data.chars().filter(|c| *c != '"').collect();
    let names: Vec<&str> = data2.split(",").collect();

    let mut count = 0;
    for name in names {
        let word_score = name.chars().map(|ch| ch as usize - 64).sum();
        if tri_numbers.contains(&word_score) {
            //println!("{} {}", name, word_score);
            count += 1;
        }
    }
    println!("{}", count);
}
// 162
