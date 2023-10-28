use std::cmp::min;

use ac_library::{Max, Segtree};
use proconio::input;

use superslice::Ext;

fn main() {
    input! {
        n:usize,
        l:usize,
        r:usize,
        x:[usize; n],
    }
    let mut st = Segtree::<Max<usize>>::new(n + 1);
    for i in 0..n {
        st.set(i, x[i]);
    }

    let mut ans = 0usize;
    let mut height = 0usize;
    while height < x[n - 1] {
        let l = height + l;
        let r = min(height + r, x[n - 1]);

        let m = st.prod(l..=r);
        if m > 0 {
            height += m;
            ans += 1;
        }
    }

    println!("{}", ans);
}
