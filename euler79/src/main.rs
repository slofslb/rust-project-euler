fn main() {
    let data = std::fs::read_to_string("keylog.txt").expect("打开文件失败");
    let data2 = data.trim().replace("\r", "");
    let lines = data2.split("\n");

    let mut set: Vec<(&str, &str)> = vec![];
    for line in lines {
        let a = &line[..1];
        let b = &line[1..=1];
        let c = &line[2..];
        if !set.contains(&(a, b)) {
            set.push((a, b));
        }
        if !set.contains(&(b, c)) {
            set.push((b, c));
        }
    }

    // generate a graphviz format file
    // graphviz.org download & install software, generate graph!
    println!("digraph G {{");
    println!("  rankdir=LR;");
    for (a, b) in set {
        println!("  {} -> {};", a, b);
    }
    println!("}}");
}
// 73162890
