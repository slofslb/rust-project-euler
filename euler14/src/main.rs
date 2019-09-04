fn main() {
    let mut max = 0;
    for num in 1..1_000_000 {
        let c = collatz(num as u64);
        if c >= max {
            max = c;
            println!("start num: {}   chain length: {}", num, max);
        }
    }
}
// 837799

fn collatz(x: u64) -> u64 {
    if x == 1 {
        return 1;
    }
    let y = if x % 2 == 0 { x / 2 } else { x * 3 + 1 };
    collatz(y) + 1
}

/*
fn main() {
    let mut v:Vec<u64> = vec![0; 1_000_000 ];
    v[1] = 1;

    let mut max = 0;
    for num in 2.. 1_000_000 {
        let a = collatz(&mut v, num as u64);
        if a > max {
            max = a;
            println!("{} {}", num, max);
        }
    }
}
// 837799

fn collatz(v: &mut Vec<u64> , x : u64) -> u64 {
    if x == 1 {return 1;}

    if x < v.len() as u64 &&  v[x as usize] > 0  {return v[x as usize];}
    else {
        let  y:u64;
        if x % 2 == 0 {
            y = collatz(v, x / 2);
        }
        else {
            y = collatz(v, x * 3 + 1);
        }
        if x < v.len() as u64 {
            v[x as usize]= y + 1;
        }
        return y+1;
    }
}
*/

#[test]
fn some() {
    let max_size = 1_000_000;
    let mut v: Vec<u64> = vec![0; max_size];
    v[1] = 1;
    collatz(&mut v, 3);
    assert_eq!(2, v[3]); // will fail
}
