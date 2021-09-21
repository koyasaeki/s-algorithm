#![allow(non_snake_case)]

fn main() {
    solve();
}

/// 探索問題 1.
fn solve() {
    let n: usize = 10;
    let a: Vec<i64> = vec![1; n];
    let v: i64 = 1;
    for i in 0..a.len() {
        if a[i] == v {
            println!("idx: {}", i);
            return;
        }
    }

    println!("-1");
}
