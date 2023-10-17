use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        N:usize,
        A:[i64; N],
    }

    let mut m = HashMap::new();

    let mut ans = 0i64;
    for a in A {
        if m.contains_key(&a) {
            let v = m.get(&a).unwrap();
            ans += v;
            m.insert(a, v + 1);
        } else {
            m.insert(a, 1i64);
        }
        // println!("{:?}", m);
    }

    println!("{}", ans);
}
