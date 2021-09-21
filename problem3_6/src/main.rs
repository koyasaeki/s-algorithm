#![allow(non_snake_case)]
use proconio::input;

fn main() {
    solve();
}

/// ABC 051 B - Sum of Three Integers.
///
/// https://atcoder.jp/contests/abc051/tasks/abc051_b
fn solve() {
    input! {
        K: usize,
        S: i64
    }

    // K * K <= 2500 * 2500 = 25 ** 2 * 10 ** 4 <= 10 ** 7
    let mut count = 0;
    for x in 0..=K {
       for y in 0..=K {
          let x = x as i64;
          let y = y as i64;
          let K = K as i64;

          let z = S - x - y;
          if z < 0 || z > K {
              continue;
          }
          count += 1;
       }
    }

    println!("{}", count);
}
