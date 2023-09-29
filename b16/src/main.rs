use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        n:usize,
        h:[i32; n]
    }

    let mut dp = vec![];

    dp.push(0);
    dp.push((h[1] - h[0]).abs());

    for i in 2..n {
        let x = (h[i - 1] - h[i]).abs();
        let y = (h[i - 2] - h[i]).abs();
        dp.push(min(dp[i - 1] + x, dp[i - 2] + y));
    }

    println!("{}", dp[n - 1]);
}
