use std::cmp::max;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s:Chars,
        mut t:Chars
    }
    s.insert(0, ' ');
    t.insert(0, ' ');

    let mut dp = vec![vec![0; t.len()]; s.len()];
    dp[0][0] = 0;

    for i in 1..s.len() {
        for j in 1..t.len() {
            if s[i] == t[j] {
                dp[i][j] = max(max(dp[i - 1][j], dp[i][j - 1]), dp[i - 1][j - 1] + 1);
            } else {
                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
        // println!("{:?}", dp[i]);
    }

    println!("{}", dp[s.len() - 1][t.len() - 1]);
}
