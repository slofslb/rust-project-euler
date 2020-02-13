use primes::PrimeSet;

fn main() {
    // 试算一些数，找找规律
    let mut max_ratio = 0_f64;
    for n in 2..3000 {
        let mut phi = 0;
        for i in 1..n {
            // 最大公约数为1，表示互质
            if gcd(i, n) == 1 {
                phi += 1;
            }
        }
        let ratio = (n as f64) / (phi as f64);
        if ratio > max_ratio {
            println!("n= {:6}  phi={:6}  n/phi= {:.4}", n, phi, ratio);
            max_ratio = ratio;
        }
    }
    // 2 * 3 * 5 * 7 * 11 * ... 找到最多的素数，乘积在1百万以内
    let mut pset = PrimeSet::new();
    let mut prod = 1;
    for p in pset.iter() {
        if prod * p > 1_000_000 {
            break;
        }
        prod *= p;
        println!("prime={} {}", p, prod);
    }
    println!("{}", prod);

    // 另一种函数式编程的写法，不易读
    let mut some_primes = (2..).filter(|&x| primes::is_prime(x));
    let mut prod = 1;
    let some_products = std::iter::repeat_with(|| {
        let tmp = prod;
        prod *= some_primes.next().unwrap();
        tmp
    })
    .take_while(|&x| x < 1_000_000)
    .collect::<Vec<_>>();
    println!("{:?}", some_products);
    println!("{:?}", some_products.last().unwrap());

    // 用itertools写起来直观一些
    let mut some_primes = (2..).filter(|&x| primes::is_prime(x));
    let result = itertools::iterate(1, |&prod| prod * some_primes.next().unwrap())
        .take_while(|&x| x < 1_000_000)
        .last()
        .unwrap();
    println!("{:?}", result);
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
