use std::cmp::max;

use proconio::input;

fn main() {
    input! {
        N:usize,
        K:i64,
        AB:[(i64, i64); N],
    }

    let mut ans = 0;
    for a in 1..=100 {
        for b in 1..=100 {
            ans = max(ans, calc(&AB, a, b, K));
        }
    }

    println!("{}", ans);
}

fn calc(ab: &Vec<(i64, i64)>, a: i64, b: i64, k: i64) -> i64 {
    let mut sum = 0;
    for &(aa, bb) in ab {
        if a <= aa && aa <= a + k && b <= bb && bb <= b + k {
            sum += 1;
        }
    }

    sum
}