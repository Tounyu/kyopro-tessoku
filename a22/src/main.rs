use std::cmp::max;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize; n-1],
        b:[usize; n-1]
    }

    let mut dp = vec![-1000000000; n];
    dp[0] = 0;
    for i in 0..n - 1 {
        dp[a[i] - 1] = max(dp[a[i] - 1], dp[i] + 100);
        dp[b[i] - 1] = max(dp[b[i] - 1], dp[i] + 150);
        // println!("{:?}", dp);
    }

    println!("{}", dp[n - 1]);
}
