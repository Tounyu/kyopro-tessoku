use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[[i32; n]; m],
    }

    let nn = 1 << n;

    let mut dp = vec![vec![1_000_000_001; nn]; m + 1];

    dp[0][0] = 0;

    for i in 1..=m {
        let c = a[i - 1].iter().enumerate().fold(0, |acc, (index, &x)| if x > 0 { acc | 1 << index as i32 } else { acc });
        // println!("{:?}, c={:b}", a[i - 1], c);
        for s in 0..nn {
            dp[i][s] = min(dp[i][s], dp[i - 1][s]);
            for k in 0..nn {
                if c | k == s {
                    dp[i][s] = min(dp[i][s], dp[i - 1][k] + 1);
                }
            }
        }
        // println!("{:?}", dp[i]);
    }

    if dp[m][nn - 1] > 1_000_000_000 {
        println!("-1");
    } else {
        println!("{}", dp[m][nn - 1]);
    }
}
