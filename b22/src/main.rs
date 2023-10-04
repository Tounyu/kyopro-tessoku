use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i64; n-1],
        b:[i64; n-2]
    }

    let mut dp = vec![100000000; n];
    dp[0] = 0;

    for i in 0..n {
        if i + 1 < n {
            dp[i + 1] = min(dp[i + 1], dp[i] + a[i]);
        }
        if i + 2 < n {
            dp[i + 2] = min(dp[i + 2], dp[i] + b[i]);
        }
    }

    println!("{}", dp[n - 1]);
}
