fn main() {
    //println!("{}", count_right_triangles(120));
    let mut max_count = 1;
    let mut max_p = 0;
    for p in 2..1000 {
        let count = count_right_triangles(p);
        if count > max_count {
            max_count = count;
            max_p = p;
            println!("p: {} max: {}", p, max_count);
        }
    }
    println!("p: {}", max_p);
}
// p: 840 max: 8

fn count_right_triangles(p: isize) -> isize {
    let mut count = 0;
    for a in 1..p {
        for b in a..p {
            let c = p - a - b;
            if c > 0 && a * a + b * b == c * c {
                count += 1;
                //print!("{{{}, {}, {}}}, ", a, b, c)
            }
        }
    }
    count
}
