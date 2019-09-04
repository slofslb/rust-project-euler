
fn main() {
    let mut v = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for i in 2..=1_000_000 {
        next_perm(&mut v);
        //println!("{} {:?}", i, v);
    }
    println!("{:?}", v);

    let v_str = v.iter().map(|x| x.to_string()).collect::<String>();
    println!("{}", v_str.parse::<u64>().unwrap());
}
// 2783915460

fn next_perm(v: &mut [u32]) {
    let mut i = v.len() - 2;
    while v[i] > v[i + 1] {
        i -= 1;
    }

    let mut j = v.len() - 1;
    while i < j && v[i] > v[j] {
        j -= 1;
    }

    swap(v, i, j);

    i += 1;
    j = v.len() - 1;
    while i < j {
        swap(v, i, j);
        i += 1;
        j -= 1;
    }
}

fn swap(v: &mut [u32], i: usize, j: usize) {
    let temp = v[i];
    v[i] = v[j];
    v[j] = temp;
}

/*
fn main() {
    let mut v: Vec<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for i in 2..=1_000_000 {
        next_perm(&mut v);
        //println!("{} {:?}", i, v);
    }
    println!("{:?}", v);

    let v_str = v.iter().map(|x| x.to_string()).collect::<String>();
    println!("{}", v_str.parse::<u64>().unwrap());
}
// 2783915460

fn next_perm(v: &mut Vec<u32>) {
    let mut i = v.len() - 2;
    while v[i] > v[i + 1] {
        i -= 1;
    }

    let mut j = v.len() - 1;
    while i < j && v[i] > v[j] {
        j -= 1;
    }

    swap(v, i, j);

    i += 1;
    j = v.len() - 1;
    while i < j {
        swap(v, i, j);
        i += 1;
        j -= 1;
    }
}

fn swap(v: &mut Vec<u32>, i: usize, j: usize) {
    let temp = v[i];
    v[i] = v[j];
    v[j] = temp;
}
*/
