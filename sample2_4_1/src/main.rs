#![allow(non_snake_case)]

fn main() {
    solve();
}

/// 最近点対問題
///
///   正の整数 N と、N 個の座標値 (x_i, y_i) (i = 0, ..., N - 1) が与えられます。
///   最も距離が近い 2 点間の距離を求めてください。
fn solve() {
    // 計算中に最も近い点間の距離を更新していく.
    let n = 4;
    let mut dist = 1_000_000_000;
    let points: Vec<[i32; 2]> = vec![[1, 1], [0, 0], [-10, 0], [-1, -1]];

    // i < j として良い
    for i in 0..n {
        for j in (i + 1)..n {
            let d = (points[i][0] - points[j][0]).pow(2) + (points[i][1] - points[j][1]).pow(2);
            if dist > d {
                dist = d
            }
        }
    }

    println!("{}", (dist as f64).sqrt());
}
