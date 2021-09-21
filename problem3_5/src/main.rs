#![allow(non_snake_case)]
use proconio::input;

fn main() {
    solve();
}

/// ABC 081 B - Shift Only.
///
/// https://atcoder.jp/contests/abc081/tasks/abc081_b
fn solve() {
    input! {
        N: usize,
        mut A: [u64; N]
    }
    // すぬけくんが操作した回数
    let mut count = 0;
    'outer: loop {
        for i in 0..N {
            if A[i] % 2 == 1 {
                break 'outer;
            } else {
                A[i] = A[i] >> 1;
            }
        }
        count += 1;
    }

    println!("{}", count);
}
