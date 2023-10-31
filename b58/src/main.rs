use ac_library::{Min, Segtree};
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        l:isize,
        r:isize,
        x:[isize; n],
    }
    let mut st = Segtree::<Min<usize>>::new(n + 1);
    let mut dp = vec![usize::MAX; n];
    dp[0] = 0;
    st.set(0, 0);
    for i in 1..n {
        if x[i] < l {
            continue;
        }
        let posl = x.lower_bound(&(&x[i] - r));
        let posr = x.upper_bound(&(&x[i] - l));

        let m = st.prod(posl..posr);
        if m != usize::MAX {
            st.set(i, m + 1);
            dp[i] = m + 1;
        }
    }

    println!("{}", dp[n - 1]);
}
