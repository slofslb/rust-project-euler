fn main() {
    let data = std::fs::read_to_string("keylog.txt").expect("打开文件失败");
    let lines = data.trim().split("\n");

    let mut kv: Vec<(&str, &str)> = vec![];
    for line in lines {
        let a = &line[..1];
        let b = &line[1..=1];
        let c = &line[2..];
        kv.push((a, b));
        kv.push((b, c));
        /*        if !kv.contains(&(a,b)) {
                    kv.push((a,b));
                }
                if !kv.contains(&(b,c)) {
                    kv.push((b,c));
                }
        */
    }

    // generate a graphviz format file
    println!("digraph G {{");
    println!("rankdir=LR;");
    for (a, b) in kv {
        println!("{} -> {};", a, b);
    }
    println!("}}");
}
// 73162890
