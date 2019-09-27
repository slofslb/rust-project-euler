fn main() {
    let mut tri: Vec<u64> = vec![];
    for i in 1..100000 {
        tri.push(i * (i + 1) / 2);
    }

    let mut penta: Vec<u64> = vec![];
    for i in 1..100000 {
        penta.push(i * (3 * i - 1) / 2);
    }

    for i in 2..30000 {
        let hex = i * (2 * i - 1);
        if tri.contains(&hex) && penta.contains(&hex) {
            println!("i: {}  hexagonal: {}", i, hex);
        }
    }
}
// i: 143  hexagonal: 40755
// i: 27693  hexagonal: 1533776805
