use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize; n-1],
        b:[usize; n-2],
    }

    let mut dp = vec![];

    dp.push(0);
    dp.push(a[0]);
    for i in 2..n {
        dp.push(min(dp[i - 1] + a[i - 1], dp[i - 2] + b[i - 2]));
    }

    println!("{}", dp[n - 1]);
}
