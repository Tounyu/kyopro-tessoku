use std::cmp::min;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s:Chars,
        mut t:Chars,
    }
    s.insert(0, ' ');
    t.insert(0, ' ');

    let mut dp = vec![vec![0; t.len()]; s.len()];
    dp[0][0] = 0;
    for i in 1..s.len() { dp[i][0] = i; }
    for j in 1..t.len() { dp[0][j] = j; }


    for i in 1..s.len() {
        for j in 1..t.len() {
            let c = if s[i] == t[j] {
                0
            } else {
                1
            };
            dp[i][j] = min(min(dp[i - 1][j] + 1, dp[i][j - 1] + 1), dp[i - 1][j - 1] + c);
        }
        // println!("{:?}", dp[i]);
    }

    println!("{}", dp[s.len() - 1][t.len() - 1]);
}
