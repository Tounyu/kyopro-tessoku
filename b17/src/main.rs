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

    let mut ans = vec![];
    let mut i = n - 1;
    ans.push(i);
    while i >= 2 {
        i = if dp[i] == dp[i - 1] + (h[i - 1] - h[i]).abs() {
            i - 1
        } else {
            i - 2
        };
        ans.push(i);
    }
    if i == 1 {
        ans.push(0);
    }

    let ans_s: Vec<String> = ans.iter().rev().map(|x| (x + 1).to_string()).collect();

    println!("{}", ans.len());
    println!("{}", ans_s.join(" "));
}
