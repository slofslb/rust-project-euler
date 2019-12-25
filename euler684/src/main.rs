fn main() {
    let mut fib = [0_u64;91];
    fib[1] = 1;
    for i in 2..=90 {
        fib[i] = fib[i-1] + fib[i-2]; 
        println!("fib({}): {}", i, fib[i]);
    }

    let mut sum = 0;
    for n in 1..=20 {
        let b = n / 9;
        let a = n % 9;
        let temp = (a+1) * 10_u32.pow(b) - 1;
        println!("n:{} temp: {}", n, temp);
        sum += temp;
    }
    println!("{}", sum);
}
