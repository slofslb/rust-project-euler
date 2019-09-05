fn main() {
    let mut t: Vec<u64> = vec![];
    for i in 1..100000 {
        t.push(i * (i + 1) / 2);
    }
    //println!("{:?}", t);

    let mut p: Vec<u64> = vec![];
    for i in 1..100000 {
        p.push(i * (3 * i - 1) / 2);
    }
    //println!("{:?}", p);

    for i in 100..30000 {
        let hex = i * (2 * i - 1);
        if t.contains(&hex) && p.contains(&hex) {
            println!("i: {}  hexagonal: {}", i, hex);
        }
    }
}
// i: 143  hexagonal: 40755
// i: 27693  hexagonal: 1533776805
