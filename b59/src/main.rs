use ac_library::{Additive, Segtree};
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }

    let mut st = Segtree::<Additive<usize>>::new(n + 1);
    let mut ans = 0usize;
    for a in a {
        ans += st.prod(a + 1..n + 1);
        st.set(a, 1);
    }

    println!("{}", ans);
}
