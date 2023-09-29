use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut x = a.clone();
    x.sort();
    x.dedup();
    let mut set: BTreeMap<usize, usize> = BTreeMap::new();
    for i in 0..x.iter().count() {
        set.insert(x[i], i + 1);
    }

    for i in 0..n {
        if let Some((_le, index)) = set.range(..=a[i]).next_back() {
            if i == 0 {
                print!("{}", index);
            } else {
                print!(" {}", index);
            }
        }
    }
    println!();
}
