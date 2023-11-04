use std::cmp::max;

use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        k:usize,
        ab:[(usize,usize); m],
    }

    let mut dp = vec![vec![-1000000000i64; n + 1]; k + 1];
    dp[0][0] = 0;
    for k in 1..=k {
        for i in 1..=n {
            for j in 0..i {
                dp[k][i] = max(dp[k][i], dp[k - 1][j] + score(&ab, j + 1, i));
            }
        }
    }

    println!("{}", dp[k][n]);
}

fn score(ab: &Vec<(usize, usize)>, l: usize, r: usize) -> i64 {
    let mut score = 0i64;
    for &(a, b) in ab {
        if l <= a && b <= r {
            score += 1;
        }
    }
    score
}
