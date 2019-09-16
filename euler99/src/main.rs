fn main() {
    let data = std::fs::read_to_string("base_exp.txt").expect("读文件失败");
    let lines = data.trim().split('\n');
    let mut max = 0_f64;
    for (i, line_str) in lines.enumerate() {
        let line_data: Vec<usize> = line_str
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let (base, exp) = (line_data[0], line_data[1]);
        let v = (exp as f64) * (base as f64).log10();
        if v > max {
            println!("{} {} {}", i + 1, line_str, v);
            max = v;
        }
    }
}
