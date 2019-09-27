fn main() {
    algo1_slow();

    let mut min_d = std::u64::MAX; // 99999999
    for k in 2..10000 {
        let pk = penta(k);
        if pk - penta(k - 1) > min_d {
            break;
        }
        for j in (1..k).rev() {
            let pj = penta(j);
            let d = pk - pj;
            if d < min_d && is_penta(d) && is_penta(pk + pj) {
                println!("j:{} k:{} pj:{} pk:{} diff:{}", j, k, pj, pk, d);
                min_d = d;
                break;
            }
        }
    }
}

fn is_penta(p: u64) -> bool {
    let t = ((1 + 24 * p) as f64).sqrt() as u64; // sqrt(b*b - 4*a*c)
    if t * t != (1 + 24 * p) {
        return false;
    }
    (t + 1) % 6 == 0
}

fn penta(n: u64) -> u64 {
    return n * (3 * n - 1) / 2;
}

fn algo1_slow() {
    let mut penta: Vec<u64> = vec![0];
    for i in 1..10000 {
        penta.push(i * (3 * i - 1) / 2);
    }

    for k in 2..3000 {
        for j in (1..k).rev() {
            let d = penta[k] - penta[j];
            let sum = penta[k] + penta[j];
            if penta.contains(&d) && penta.contains(&sum) {
                println!("j:{} k:{} pj:{} pk:{} diff:{}", j, k, penta[j], penta[k], d);
            }
        }
    }
}
