fn main() {
    //let f  = vec![0..10].iter().map(|&x| fac(x));//.collect();
    //let mut f = vec![0..10].iter_mut().for_each(|el| *el = fac(*el));//.collect();
    let mut f = vec![1; 10];
    for i in 0..10 {
        f[i] = fac(i as u32);
    }
    println!("{:?}", f);

    for i in 3..999999 {
        let mut sum_fac = 0;
        
        for d in i.to_string()
         .chars()
         .map(|x| x.to_digit(10).unwrap()) {
             sum_fac += f[d as usize];
         }
         
        if i == sum_fac {
            println!("{}", i);
        } 
    }
}

fn fac(n: u32) -> u32{
    if n <= 1 {return 1;}
    return n * fac(n-1);
}

