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
            m.remove(&v);
        }
        if q1 == 3 {
            if let Some(le) = m.range(v..).next() {
                println!("{}", le);
            } else {
                println!("-1");
            }
        }
    }
}
