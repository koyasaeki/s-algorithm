#![allow(non_snake_case)]

fn main() {
    sample1();
    sample2();
}

/// 計算量が O(N) の例.
fn sample1() {
    let mut n = 0;

    let N = 100;
    for i in 1..=N {
        n += i;
    }

    println!("{}", n);
}

/// 計算量が O(N^2) の例.
fn sample2() {
    let mut n = 0;

    let N = 100;
    for i in 1..=N {
        for j in 1..=N {
            n += i + j;
        }
    }
    println!("{}", n);
}
