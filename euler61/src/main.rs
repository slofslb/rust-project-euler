fn main() {
    let tri = gen_poly_numbers(1000, 9999, |n| n * (n + 1) / 2);
    let sqr = gen_poly_numbers(1000, 9999, |n| n * n);
    let pent = gen_poly_numbers(1000, 9999, |n| n * (3 * n - 1) / 2);
    let hex = gen_poly_numbers(1000, 9999, |n| n * (2 * n - 1));
    let hep = gen_poly_numbers(1000, 9999, |n| n * (5 * n - 3) / 2);
    let oct = gen_poly_numbers(1000, 9999, |n| n * (3 * n - 2));

    let mut v = vec![];
    println!("triangle: {:?}", tri);
    for &e in tri.iter() {
        v.push((1, head(e), tail(e)));
    }

    println!("square: {:?}", sqr);
    for &e in sqr.iter() {
        v.push((2, head(e), tail(e)));
    }

    println!("pentagonal: {:?}", pent);
    for &e in pent.iter() {
        v.push((3, head(e), tail(e)));
    }

    println!("hexagonal: {:?}", hex);
    for &e in hex.iter() {
        v.push((4, head(e), tail(e)));
    }

    println!("heptagonal: {:?}", hep);
    for &e in hep.iter() {
        v.push((5, head(e), tail(e)));
    }

    println!("octagonal: {:?}", oct);
    for &e in oct.iter() {
        v.push((6, head(e), tail(e)));
    }

    //find(&v, 45, 10, &[1,2,3,4,5]);

    let mut samples = vec![];
    for i in 0..100 {
        let mut sample = vec![];
        let a = tri[rand::random::<usize>() % tri.len()];
        sample.push(a);
        let b = sqr[rand::random::<usize>() % sqr.len()];
        sample.push(b);
        let c = pent[rand::random::<usize>() % pent.len()];
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
                sample_new[index] = sqr[rand::random::<usize>() % sqr.len()];
            } else if index == 2 {
                sample_new[index] = pent[rand::random::<usize>() % pent.len()];
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
                    println!("{:?} {}", sample_new, sample_new.iter().sum::<u32>());
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
*/
fn gen_poly_numbers(start: u32, end: u32, f: fn(u32) -> u32) -> Vec<u32> {
    (1..)
        .map(f)
        .filter(|&n| n >= start)
        .take_while(|&n| n <= end)
        .collect()
}
