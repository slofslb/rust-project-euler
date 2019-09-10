fn main() {
    let data = std::fs::read_to_string("keylog.txt").expect("打开文件失败");
    let lines = data.split("\n");

    let mut kv : Vec<(&str, &str)> = vec![];

    for line in lines {
        if line.len() < 3 {continue;}
        let a = &line[..1];
        let b = &line[1..2];
        let c = &line[2..];
        if !kv.contains(&(a,b)) {
            kv.push((a,b));
        }
        if !kv.contains(&(b,c)) {
            kv.push((b,c));
        }
    }
        println!("{:?}", kv );
}
