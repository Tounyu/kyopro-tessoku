use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize; n],
    }

    if n == 1 {
        if a[0] == k {
            println!("Yes");
        } else {
            println!("No");
        }
        return;
    }

    let mut l = vec![];
    let mut r = vec![];
    let m = n / 2;

    for i in 1..=m {
        for comb in (0..m).combinations(i) {
            // println!("{:?}", comb);
            let sum = comb.iter().fold(0, |acc, &x| acc + a[x]);
            if sum == k {
                println!("Yes");
                return;
            }
            l.push(sum);
        }
    }
    for i in 1..=(n - m) {
        for comb in (m..n).combinations(i) {
            // println!("{:?}", comb);
            let sum = comb.iter().fold(0, |acc, &x| acc + a[x]);
            if sum == k {
                println!("Yes");
                return;
            }
            r.push(sum);
        }
    }

    l.sort();
    let mut set: BTreeSet<usize> = BTreeSet::new();
    for v in r {
        set.insert(v);
    }
    for lv in l {
        let t = k - lv;
        if let Some(&le) = set.range(..=t).next_back() {
            if le == t {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
