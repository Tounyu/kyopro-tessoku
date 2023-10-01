use std::cmp::max;

use proconio::input;

fn main() {
    input! {
        n:usize,
        w:usize,
        mut wv:[(usize, i64); n]
    }
    wv.insert(0, (0, 0));
    let mut dp: Vec<Vec<i64>> = vec![vec![-1; w + 1]; n + 1];

    for i in 1..=n {
        for j in 0..=w {
            if j < wv[i].0 {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = max(dp[i - 1][j], dp[i - 1][j - wv[i].0] + wv[i].1);
            }
        }
    }

    let ans = dp[n].iter().fold(0, |mx, &x| max(mx, x + 1));

    println!("{}", ans);
}
