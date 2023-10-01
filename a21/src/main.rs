use std::cmp::max;

use proconio::input;

fn main() {
    input! {
        n:usize,
        mut pa:[(usize, i32); n]
    }
    pa.insert(0, (0, 0));
    pa.push((0, 0));

    let mut dp = vec![vec![0; n + 1]; n + 1];
    dp[1][n] = 0;

    for len in (0..=n - 2).rev() {
        for l in 1..=n - len {
            let r = l + len;
            // println!("{},{}", l, r);
            let sl = if l <= pa[l - 1].0 && pa[l - 1].0 <= r {
                pa[l - 1].1
            } else {
                0
            };
            let sr = if l <= pa[r + 1].0 && pa[r + 1].0 <= r {
                pa[r + 1].1
            } else {
                0
            };

            if l == 1 {
                dp[l][r] = dp[l][r + 1] + sr;
            } else if r == n {
                dp[l][r] = dp[l - 1][r] + sl;
            } else {
                dp[l][r] = max(dp[l - 1][r] + sl, dp[l][r + 1] + sr);
            }
        }
    }

    let mut ans = 0;
    for i in 1..=n {
        ans = max(ans, dp[i][i]);
    }
    println!("{}", ans);
}
