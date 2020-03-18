fn main() {
    //let n = 13;
    let mut count = 0;
    for n in 2..=10000 {
        let period = period_square_root(n);
        if period % 2 == 1 {
            count += 1;
        }
        println!("{} period: {}", n, period);
    }
    println!("{}", count);
}

fn period_square_root(n: usize) -> usize {
    let sqrt_n = (n as f32).sqrt();
    if (sqrt_n as usize).pow(2) == n {
        return 0;
    }

    let mut a = vec![];
    let mut b = vec![];
    let mut t = vec![];

    a.push(sqrt_n as usize);
    b.push(sqrt_n as usize);
    t.push(1);
    //println!("a: {} b: {} t: {}", a[0], b[0], t[0]);

    for i in 1.. {
        t.push((n - b[i - 1] * b[i - 1]) / t[i - 1]);
        a.push(((sqrt_n + b[i - 1] as f32) / (t[i] as f32)) as usize);
        b.push(a[i] * t[i] - b[i - 1]);
        //println!("a: {} b: {} t: {}", a[i], b[i], t[i]);
        if t[i] == t[0] {
            return i;
        }
    }
    0
}
