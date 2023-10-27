use std::cmp::max;

use proconio::input;

fn main() {
    input! {
        n:usize,
        mut td:[(usize,usize); n],
    }
    td.sort_by_key(|&a| a.1);
    let mut dp = vec![vec![-1i64; 1449]; 109];
    dp[0][0] = 0;
    for i in 1..=n {
        for j in 0..=1440 {
            if td[i - 1].1 < j || j < td[i - 1].0 {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = max(dp[i - 1][j], dp[i - 1][j - td[i - 1].0] + 1);
            }
        }
    }
    let ans = dp[n].iter().fold(0i64, |acc, &x| max(acc, x));
    println!("{ans}");
}
