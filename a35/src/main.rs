use std::cmp::{max, min};

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }

    let mut dp = vec![vec![0; n]; n];

    for i in 0..n {
        dp[n - 1][i] = a[i];
    }

    for i in (0..n - 1).rev() {
        for j in 0..=i {
            if i % 2 == 0 {
                dp[i][j] = max(dp[i + 1][j], dp[i + 1][j + 1]);
            } else {
                dp[i][j] = min(dp[i + 1][j], dp[i + 1][j + 1]);
            }
        }
    }

    println!("{}", dp[0][0]);
}
