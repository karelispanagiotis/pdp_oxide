use proconio::input;
use std::cmp::min;

fn main() {
    input! { n: usize, mut a: [i64; n] };
    a.insert(0, i64::MAX);
    let mut dp = vec![vec![0; a.len() + 1]; 2];
    for i in (1..a.len()).rev() {
        for j in 0..=i {
            if a[j] >= a[i] {
                dp[i % 2][j] = min(dp[(i + 1) % 2][i], 2 * a[i] + dp[(i + 1) % 2][j]);
            } else {
                dp[i % 2][j] = min(a[i] - a[j] + dp[(i + 1) % 2][j], 2 * a[i] + dp[(i + 1) % 2][j]);
            }
        }
    }
    println!("{}", dp[1][0]);
}
