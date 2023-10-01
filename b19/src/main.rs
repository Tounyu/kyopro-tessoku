use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        n:usize,
        w:i64,
        mut wv:[(i64, usize); n]
    }

    let v_max = 1000 * 100;

    wv.insert(0, (0, 0));

    let mut dp: Vec<Vec<i64>> = vec![vec![w + 1; v_max + 1]; n + 1];
    dp[0][0] = 0;

    for i in 1..=n {
        for j in 0..=v_max {
            if j < wv[i].1 {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = min(dp[i - 1][j], dp[i - 1][j - wv[i].1] + wv[i].0);
            }
        }
    }

    let mut ans = v_max;
    while dp[n][ans] > w {
        ans -= 1;
    }

    println!("{}", ans);
}
