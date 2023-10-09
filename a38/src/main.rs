use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        D:usize,
        N:usize,
        LRH:[(usize,usize,i32); N],
    }

    let mut hmax = vec![24; D];

    for i in 0..D {
        for &(l, r, h) in &LRH {
            if l <= i + 1 && i + 1 <= r {
                hmax[i] = min(hmax[i], h);
            }
        }
    }

    let ans = hmax.iter().fold(0, |acc, &h| acc + h);
    println!("{}", ans);
}
