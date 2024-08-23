use std::{
    io::{self, Write},
    usize,
};

fn main() {
    println!("u32\t{}", u32::MAX);
    println!("u64\t{}", u64::MAX);
    println!("u128\t{}", u128::MAX);
    println!("i32\t{}", i32::MAX);
    println!("i64\t{}", i64::MAX);
    println!("i128\t{}", i128::MAX);
    let mut memo = vec![0; 200];
    memo[1] = 1;
    for n in 0..100 {
        let fibn = memfib(n, &mut memo);
        println!("The {n}th fibonacci is {fibn}");
    }

    loop {
        // Need to flush stdout, as it's often line buffered, stalling print!
        print!("Enter a value: n=");
        let _ = io::stdout().flush();
        let mut user_n = String::new();
        io::stdin().read_line(&mut user_n).expect("Bad input?");
        // println!("user_n = {user_n:?}");
        let user_n: u16 = match user_n.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        let fibn = memfib(user_n, &mut memo);
        println!("The {user_n}th fibonacci is {fibn}");
    }
}

/// Generate the nth fibonnaci number (recursively)
fn _fib(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return _fib(n - 1) + _fib(n - 2);
}

/// Memoised: Generate the nth fibonnaci number (recursively)
fn memfib(n: u16, m: &mut Vec<u128>) -> u128 {
    if n < 2 {
        return n.into();
    }
    // check memo
    let index = n as usize;
    if n > 0 && m[index] > 0 {
        return m[index];
    }
    let left = memfib(n - 2, m);
    let right = memfib(n - 1, m);
    if right >= u128::MAX / 2 {
        panic!("size limit will be exceeded");
    }
    let res = left + right;
    m[index] = res;
    return res;
}
