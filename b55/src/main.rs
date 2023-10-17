use std::cmp::min;
use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        Q:usize,
    }
    let mut m = BTreeSet::new();
    for _i in 0..Q {
        input! {
            q1:usize,
            v:i64,
        }
        if q1 == 1 {
            m.insert(v);
        }
        if q1 == 2 {
            let mut ans = 1000000000000i64;
            if let Some(ge) = m.range(v..).next() {
                ans = min(ans, (v - ge).abs())
            }
            if let Some(le) = m.range(..=v).next_back() {
                ans = min(ans, (v - le).abs())
            }
            if ans > 100000000000i64 {
                println!("-1");
            } else {
                println!("{}", ans);
            }
        }
    }
}
