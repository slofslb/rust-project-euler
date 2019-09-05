fn main() {
    let mut max = 0;
    for p in 2..1000 {
        let count = count_right_triangles(p);
        if count > max {
            max = count;
            println!("p: {} max: {}", p, max);
        }
    } 
    println!("max: {}", max);
}
// p: 840 max: 8

fn count_right_triangles(p: isize) -> isize {
    let mut count = 0;
    for a in 1..p {
        for b in a..p {
            let c = p - a - b;
            if c <= 0 {continue;}
            if a*a + b*b == c*c {
                count += 1;
            } 
        }
    }
    count
}