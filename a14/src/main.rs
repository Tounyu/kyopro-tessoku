use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize; n],
        b:[usize; n],
        c:[usize; n],
        d:[usize; n],
    }

    let mut p = vec![];
    let mut q = vec![];
    for i in 0..n {
        for j in 0..n {
            p.push(a[i] + b[j]);
            q.push(c[i] + d[j]);
        }
    }

    p.sort();
    let mut set: BTreeSet<usize> = BTreeSet::new();
    for pv in p {
        set.insert(pv);
    }
    for qv in q {
        let t = k - qv;
        if let Some(&le) = set.range(..=t).next_back() {
            if le == t {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
