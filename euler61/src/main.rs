fn main() {
    let triangle_num = |n: u32| -> u32 { n * (n + 1) / 2 };

    let tri: Vec<u32> = (1..)
        .map(triangle_num)
        .filter(|&n| n > 1000)
        .take_while(|&n| n < 9999)
        .collect();
    println!("triangle: {:?}", tri);
    for &e in tri.iter() {
        v.push((1, head(e), tail(e)));
    }

    let squ: Vec<u32> = (1..)
        .map(|n| n * n)
        .filter(|&n| n > 1000)
        .take_while(|&n| n < 9999)
        .collect();
    println!("square: {:?}", squ);
    for &e in squ.iter() {
        v.push((2, head(e), tail(e)));
    }

    let pen: Vec<u32> = (1..)
        .map(|n| n * (3 * n - 1) / 2)
        .filter(|&n| n > 1000)
        .take_while(|&n| n < 9999)
        .collect();
    println!("pentagonal: {:?}", pen);
    for &e in pen.iter() {
        v.push((3, head(e), tail(e)));
    }

    let hex: Vec<u32> = (1..)
        .map(|n| n * (2 * n - 1))
        .filter(|&n| n > 1000)
        .take_while(|&n| n < 9999)
        .collect();
    println!("hexagonal: {:?}", hex);
    for &e in hex.iter() {
        v.push((4, head(e), tail(e)));
    }

    let hep: Vec<u32> = (1..)
        .map(|n| n * (5 * n - 3) / 2)
        .filter(|&n| n > 1000)
        .take_while(|&n| n < 9999)
        .collect();
    println!("heptagonal: {:?}", hep);
    for &e in hep.iter() {
        v.push((5, head(e), tail(e)));
    }

    let oct: Vec<u32> = (1..)
        .map(|n| n * (3 * n - 2))
        .filter(|&n| n > 1000)
        .take_while(|&n| n < 9999)
        .collect();
    println!("octagonal: {:?}", oct);
    for &e in oct.iter() {
        v.push((6, head(e), tail(e)));
    }

    //println!("{:?}", v);

    //find(&v, 45, 10, &[1,2,3,4,5]);

    let mut samples = vec![];
    for i in 0..100 {
        let mut sample = vec![];
        let a = tri[rand::random::<usize>() % tri.len()];
        sample.push(a);
        let b = squ[rand::random::<usize>() % squ.len()];
        sample.push(b);
        let c = pen[rand::random::<usize>() % pen.len()];
        sample.push(c);
        let d = hex[rand::random::<usize>() % hex.len()];
        sample.push(d);
        let e = hep[rand::random::<usize>() % hep.len()];
        sample.push(e);
        let f = oct[rand::random::<usize>() % oct.len()];
        sample.push(f);
        println!("{:?}", sample);

        samples.push(sample);
    }

    let reproduce_num = 16;
    let mut max = -100;
    for gen in 0.. {
        for _i in 0..reproduce_num * reproduce_num {
            let sample = &samples[rand::random::<usize>() % samples.len()];
            let index = rand::random::<usize>() % 6;
            let mut sample_new = sample.clone();
            if index == 0 {
                sample_new[index] = tri[rand::random::<usize>() % tri.len()];
            } else if index == 1 {
                sample_new[index] = squ[rand::random::<usize>() % squ.len()];
            } else if index == 2 {
                sample_new[index] = pen[rand::random::<usize>() % pen.len()];
            } else if index == 3 {
                sample_new[index] = hex[rand::random::<usize>() % hex.len()];
            } else if index == 4 {
                sample_new[index] = hep[rand::random::<usize>() % hep.len()];
            } else {
                sample_new[index] = oct[rand::random::<usize>() % oct.len()];
            }

            if !samples.contains(&sample_new) {
                let ev = eval(&sample_new);
                if ev == 12 {
                    println!("{:?} {}", samples[0], samples[0].iter().sum::<u32>());
                    return;
                }
                if ev > max {
                    max = ev;
                    samples.insert(0, sample_new);
                } else if ev == max - 2 {
                    //println!("insert ");
                    samples.insert(reproduce_num / 4, sample_new);
                } else if ev == max - 4 {
                    //println!("insert ");
                    samples.insert(reproduce_num / 2, sample_new);
                } else {
                    samples.push(sample_new);
                }
            }
        }
        // 只保留前面几个优质的
        if samples.len() > reproduce_num {
            for p in (reproduce_num..samples.len()).rev() {
                samples.remove(p);
            }
        }
        if gen % 100 == 0 {
            println!("{:?}  {:?} {}", samples[0], samples[reproduce_num - 1], max);
        }
    }
}
//[8256, 5625, 2882, 8128, 2512, 1281]

fn head(n: u32) -> u32 {
    n / 100
}

fn tail(n: u32) -> u32 {
    n % 100
}

fn eval(sample: &[u32]) -> i32 {
    let mut count = 0;
    for i in 0..6 {
        if head(sample[i]) == 0 || tail(sample[i]) == 0 {
            count -= 1000;
            break;
        }
        for j in 0..6 {
            if i == j {
                continue;
            }

            if head(sample[i]) == tail(sample[j]) && tail(sample[i]) == head(sample[j]) {
                count -= 100;
            }
            if head(sample[i]) == tail(sample[j]) {
                count += 1;
            }
            if tail(sample[i]) == head(sample[j]) {
                count += 1;
            }
            if head(sample[i]) == head(sample[j]) {
                count -= 5;
            }
            if tail(sample[i]) == tail(sample[j]) {
                count -= 5;
            }
        }
    }
    count
}
/*
fn find(v: &Vec<(u32,u32,u32)>, start: u32, end: u32, poly_type: &[u32]) -> Vec<(u32,u32,u32)> {
    for (head, tail, type) in v.iter() {
        if poly_type.contains(type) && head == start {
            let t = poly_type.clone().remove(&type);
            let result = find(v, tail, end, &t);
            if result.len() > 0 {
                return result;
            }
        }
    }
    vec![]
}

fn gen_poly_numbers(start:u32, end:u32, F:&Fn(u32)->u32) {
     (1..)
    .map(F)
    .filter(|&n| n >= start)
    .take_while(|&n| n <= end)
    .collect()
}
