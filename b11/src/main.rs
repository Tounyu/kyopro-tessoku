use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize; n],
        q:usize,
        x:[usize; q],
    }

    a.sort();
    let mut set: BTreeMap<usize, usize> = BTreeMap::new();
    for i in 0..n {
        set.insert(a[i], i + 1);
    }

    for xv in x {
        if let Some((_lt, index)) = set.range(..xv).next_back() {
            println!("{}", index);
        } else {
            println!("{}", 0);
        }
    }
}
