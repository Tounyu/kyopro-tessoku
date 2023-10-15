use std::cmp::max;

use proconio::input;

fn main() {
    input! {
        N:usize,
        L:i64,
        AB:[(i64,char); N],
    }

    let mut ans = 0;
    for (a, b) in AB {
        if b == 'E' {
            ans = max(ans, L - a);
        } else {
            ans = max(ans, a);
        }
    }

    println!("{}", ans);
}
