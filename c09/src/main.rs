use std::cmp::max;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize; n]
    }
    let mut dp = vec![0usize; n];
    for i in 0..n {
        if i == 0 {
            dp[i] = a[0];
        } else if i == 1 {
            dp[i] = max(dp[i - 1], a[i]);
        } else {
            dp[i] = max(dp[i - 2] + a[i], dp[i - 1]);
        }
    }

    println!("{}", dp[n - 1]);
}
