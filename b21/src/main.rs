use std::cmp::max;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n:usize,
        s:Chars,
    }

    let mut dp = vec![vec![0; n]; n];

    for len in 0..n {
        for l in 0..n - len {
            let r = l + len;
            if len == 0 {
                dp[l][r] = 1;
                continue;
            }
            if len == 1 {
                if s[l] == s[r] {
                    dp[l][r] = 2;
                } else {
                    dp[l][r] = 1;
                }
                continue;
            }

            if s[l] == s[r] {
                dp[l][r] = max(dp[l][r], dp[l + 1][r - 1] + 2);
            }
            dp[l][r] = max(dp[l][r], max(dp[l + 1][r], dp[l][r - 1]));
        }
    }

    // for i in 0..n {
    //     println!("{:?}", dp[i]);
    // }

    println!("{}", dp[0][n - 1]);
}
