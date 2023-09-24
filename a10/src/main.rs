use std::cmp::max;

use proconio::input;

fn main() {
    let size = 100009;
    input! {
        n: usize,
        a: [usize; n],
        d: usize,
        lr: [(usize, usize); d],
    }

    let mut cummaxl = vec![0; size];
    let mut cummaxr = vec![0; size];

    cummaxl[1] = a[0];
    for i in 2..=n {
        cummaxl[i] = max(cummaxl[i - 1], a[i - 1]);
    }

    cummaxr[n] = a[n - 1];
    for i in (1..n).rev() {
        cummaxr[i] = max(cummaxr[i + 1], a[i - 1]);
    }

    for (l, r) in lr {
        let ans = max(cummaxl[l - 1], cummaxr[r + 1]);
        println!("{}", ans);
    }
}


