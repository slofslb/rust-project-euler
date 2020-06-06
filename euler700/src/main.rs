fn main() {
    const INC: u64 = 1504170715041707_u64;
    const MOD: u64 = 4503599627370517_u64;

    // 试验的过程
    let mut x = 0_u64;
    let mut min = INC;
    for n in 1_u64..9999999 {
        x = (x + INC) % MOD;
        if x <= min {
            min = x;
            println!("{:20} {:20}", n, x);
        }
    }

    // 根据发现的规律，进行求解
    let mut sum = INC + 8912517754604_u64 + 2044785486369_u64 + 1311409677241_u64;
    let mut n_max = 2021_u64;
    let mut max = 4502866251561389_u64;
    let mut n = 2527_u64;
    let mut x = 1311409677241_u64;
    let mut min = x;

    while min > 0 {
        let temp_n = n + n_max;
        let temp_x = (x + max) % MOD;
        if temp_x <= min {
            n = temp_n;
            x = temp_x;
            min = x;
            sum += x;
            println!("{:20} {:12} {:20}", n, x, sum);
        }
        if temp_x > max {
            n_max = temp_n;
            max = temp_x;
            //println!("max: {} {} ", n_max, max);
        }
    }

    // 第三种：最优美的算法
    let inc = 1504170715041707_u64;
    let modular = 4503599627370517_u64;
    let mut low = inc;
    let mut high = inc;
    let mut sum = inc;
    while low > 0 {
        let next = (low + high) % modular;
        if next < low {
            low = next;
            sum += low;
            println!("{:20} {:20}", low, sum);
        } else {
            high = next;
        }
    }
    println!("{}", sum);
}
// 1517926517777556
