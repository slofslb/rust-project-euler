// 借鉴这个
// -*- compile-command: "rustc -o problem_051_rs problem_051.rs" -*-
// Copyright (c) 2016 Michael Caldwell

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }

    let limit = (n as f64).sqrt() as u64;
    let mut i = 3;
    while i <= limit {
        if n % i == 0 { return false; }
        i += 2;
    }
    return true;
}

fn main() {
    println!("Project Euler - Problem 51");
    println!("Find the smallest prime which, by replacing part of the number with the same digit, is part of an eight prime value family.\n");

    // Making an assumption that 6 digit number with 3 repeated digits
    for n in 100000..999999 {
        if is_prime(n) == false { continue; }

        // Convert to array
        let temp = [n/100000,
                    n/10000 % 10,
                    n/1000 % 10,
                    n/100 % 10,
                    n/10 % 10,
                    n % 10];

        // Look for 3 repeated numbers
        let mut digit: i32 = 0;
        for i in 0..4 {
            digit = 0;
            for j in i..6 {
                if temp[i] == temp[j] {
                    digit += 1;
                }
            }
            if digit == 3 {
                digit = temp[i] as i32;
                break;
            } else {
                digit = -1;
            }
        }

        if digit == -1 { continue; }

        // replace repeated numbers with 0-9
        let mut count = 0;
        for i in 0..10 {
            let mut new = 0;
            for j in 0..6 {
                new *= 10;
                if temp[j] == digit as u64{
                    new += i;
                } else {
                    new += temp[j];
                }
            }

            // check if prime
            if is_prime(new) && new > 100000 {
                count += 1;
            }
        }

        // Check if > 8 primes made
        if count >= 8 {
            println!("{}", n);
            break;
        }
    }
}