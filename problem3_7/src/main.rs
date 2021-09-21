#![allow(non_snake_case)]
use proconio::input;

fn main() {
    solve();
}

/// ABC 045 C - たくさんの数式.
///
/// https://atcoder.jp/contests/abc045/tasks/arc061_a
fn solve() {
    input! {
        S: String
    }
    let N = S.len();

    let mut sum = 0;
    for bit in 0..(1 << (N-1)) {
        // 分割された数字
        let mut sep_num = S[0..1].parse::<i64>().unwrap();

        for i in 0..(N-1) {
            // 左から i + 1 番目の数字を取得する.
            let ith_num = S[(i+1)..(i+2)].parse::<i64>().unwrap();

            // i 番目 と i + 1 番目の間に + を差し込む場合
            // 分割された数字が決まるので、足し込む。
            if bit & (1 << i) != 0 {
                sum += sep_num;
                sep_num = 0;
            }

            // 分割された数字を作成する。
            sep_num = 10 * sep_num + ith_num;
        }

        // 右側でのこった数字を足す。
        sum += sep_num;
    }

    println!("{}", sum);
}
