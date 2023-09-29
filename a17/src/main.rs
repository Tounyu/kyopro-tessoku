use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize; n-1],
        b:[usize; n-2],
    }

    let mut dp = vec![];

    dp.push(0);
    dp.push(a[0]);
    for i in 2..n {
        dp.push(min(dp[i - 1] + a[i - 1], dp[i - 2] + b[i - 2]));
    }

    let mut ans = vec![];
    let mut index = n - 1;
    ans.push(index);
    while index >= 2 {
        index = if dp[index] == dp[index - 1] + a[index - 1] {
            index - 1
        } else {
            index - 2
        };
        ans.push(index);
    }
    if index == 1 {
        ans.push(0);
    }

    let ans_s: Vec<String> = ans.iter().rev().map(|x| (x + 1).to_string()).collect();

    println!("{}", ans.len());
    println!("{}", ans_s.join(" "));
}
