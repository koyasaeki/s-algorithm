#![allow(non_snake_case)]

fn main() {
    solve();
}

/// ペア和の K 以上の中での最小値.
///
/// N 個の整数 a_0, ..., a_(N-1) と、N 個の整数 b_0, ..., b_(N-1) が
/// 与えられる。２組の整数列からそれぞれ１個ずつを選んで和を取る。
/// その和として考えられる値のうち、整数 K 以上の範囲内での最小値を求める。
fn solve() {
    let N = 3;
    let K = 10;
    let a = [8, 5, 4];
    let b = [4, 1, 9];

    let mut minimum = std::i64::MAX;
    for i in 0..N {
        for j in 0..N {
            let v = a[i] + b[j];
            if v < K {
                continue;
            }
            if v > minimum {
                continue;
            }
            minimum = v;
        }
    }
    println!("{}", minimum);
}
