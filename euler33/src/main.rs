fn main() {
    for ab in 10..100 {
        let a = ab / 10;
        let b = ab % 10;
        for cd in (ab+1)..100 {
            let c = cd / 10;
            let d = cd % 10;
            if b == c && ab * d == cd * a {
                println!("{} / {} = {} / {}", ab, cd, a, d);
            } 
        }
    }
}
/*
16 / 64 = 1 / 4
19 / 95 = 1 / 5
26 / 65 = 2 / 5
49 / 98 = 4 / 8 = 1 / 2
4*5*5 = 100
*/