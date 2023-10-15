use std::cmp::max;

use proconio::input;

fn main() {
    input! {
        N:usize,
        AB:[(i64,i64);N],
    }

    let mut ans = 0;
    for i in 0..=1 {
        for j in 0..=1 {
            ans = max(ans, calc(&AB, i, j));
        }
    }

    println!("{}", ans);
}

fn calc(ab: &Vec<(i64, i64)>, i: i32, j: i32) -> i64 {
    let mut score = 0i64;
    for &(a, b) in ab {
        let sa = if i == 0 {
            a
        } else {
            -a
        };
        let sb = if j == 0 {
            b
        } else {
            -b
        };

        if sa + sb > 0 {
            score += sa + sb;
        }
    }
    score
}