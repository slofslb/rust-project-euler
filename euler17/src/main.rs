fn main() {
    let mut sum = 0;
    for n in 1..=1000 {
        let s = remove_space(&english_number(n));
        sum += s.len();
        // println!("{}: {} {}", n, english_number(n), s.len());
    }
    println!("{}", sum);
}

fn english_number(n: usize) -> String {
    let list0_9 = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    if n <= 9 {
        return list0_9[n].to_string();
    }
    if n <= 19 {
        let list = vec![
            "ten",
            "eleven",
            "twelve",
            "thirteen",
            "fourteen",
            "fifteen",
            "sixteen",
            "seventeen",
            "eighteen",
            "nineteen",
        ];
        return list[n - 10].to_string();
    }
    if n <= 99 {
        let a: usize = n / 10; // 十位
        let b: usize = n % 10;
        let list = vec![
            "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
        ];
        let str = list[a].to_string();
        if b > 0 {
            return str + "-" + &english_number(b);
        }
        return str;
    }
    if n <= 999 {
        let a: usize = n / 100; // 百位
        let b: usize = n % 100;
        let str = list0_9[a].to_string() + " hundred";
        if b > 0 {
            return str + " and " + &english_number(b);
        }
        return str;
    }
    if n == 1000 {
        return "one thousand".to_string();
    }
    return "unknown".to_string();
}

fn remove_space(s: &str) -> String {
    s.chars().filter(|c| *c != ' ' && *c != '-').collect()
}
