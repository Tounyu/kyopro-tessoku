#![allow(non_snake_case)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        N:usize,
        S:Chars,
    }

    let mut hl = vec![1; N];
    for i in 0..N - 1 {
        if S[i] == 'A' {
            hl[i + 1] = hl[i] + 1;
        }
    }
    let mut hr = vec![1; N];
    for i in (0..N - 1).rev() {
        if S[i] == 'B' {
            hr[i] = hr[i + 1] + 1;
        }
    }

    let mut ans = 0;
    for i in 0..N {
        ans += hr[i].max(hl[i]);
    }
    println!("{}", ans);
}
