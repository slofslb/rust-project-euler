fn main() {
    let triangle_num = |n: u32| -> u32 { n * (n + 1) / 2 };

    let tri: Vec<u32> = (1..)
        .map(triangle_num)
        .filter(|&n| n > 1000)
        .take_while(|&n| n < 9999)
        .collect();
    println!("triangle: {:?}", tri);

    let squ: Vec<u32> = (1..)
        .map(|n| n * n)
        .filter(|&n| n > 1000)
        .take_while(|&n| n < 9999)
        .collect();
    println!("square: {:?}", squ);

    let pen: Vec<u32> = (1..)
        .map(|n| n * (3 * n - 1) / 2)
        .filter(|&n| n > 1000)
        .take_while(|&n| n < 9999)
        .collect();
    println!("pentagonal: {:?}", pen);

    let hex: Vec<u32> = (1..)
        .map(|n| n * (2 * n - 1))
        .filter(|&n| n > 1000)
        .take_while(|&n| n < 9999)
        .collect();
    println!("hexagonal: {:?}", hex);

    let hep: Vec<u32> = (1..)
        .map(|n| n * (5 * n - 3) / 2)
        .filter(|&n| n > 1000)
        .take_while(|&n| n < 9999)
        .collect();
    println!("heptagonal: {:?}", hep);

    let oct: Vec<u32> = (1..)
        .map(|n| n * (3 * n - 2))
        .filter(|&n| n > 1000)
        .take_while(|&n| n < 9999)
        .collect();
    println!("octagonal: {:?}", oct);

    /*
    for n in 2..1000 {
        let oct = n * (3 * n - 2);
        if oct > 9999 {
            break;
        }
        println!("{}", oct);
    }
    */
}

fn gen_poly_numbers(start:u32, end:u32, F:&Fn(u32)->u32) {
     (1..)
    .map(F)
    .filter(|&n| n >= start)
    .take_while(|&n| n <= end)
    .collect()
}
